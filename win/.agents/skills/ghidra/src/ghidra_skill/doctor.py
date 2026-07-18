"""Environment doctor: detect dependencies and emit actionable guidance.

Never mutates the system. For each dependency it reports status, impact, the
official URL, per-OS install commands (only for detected package managers, plus
a manual route), and a post-install verification command. Also reports the
native isolation controls and sets `dynamic_ready`.
"""
from __future__ import annotations

import os
import platform
import re
import shutil
import subprocess
from pathlib import Path
from typing import Any

# ---- OS / package-manager detection --------------------------------------

def _os_kind() -> str:
    s = platform.system().lower()
    if s.startswith("linux"):
        return "linux"
    if s == "darwin":
        return "macos"
    if s.startswith("win"):
        return "windows"
    return "other"


def _detect_pkg_managers(os_kind: str) -> list[str]:
    candidates = {
        "linux": ["apt-get", "dnf", "pacman"],
        "macos": ["brew"],
        "windows": ["winget", "choco"],
    }.get(os_kind, [])
    return [c for c in candidates if shutil.which(c)]


def _run(cmd: list[str], timeout: float = 8.0) -> tuple[int, str]:
    env = dict(os.environ, LC_ALL="C", LANG="C")
    try:
        p = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout, env=env)
        return p.returncode, (p.stdout + p.stderr)
    except (OSError, subprocess.SubprocessError):
        return 127, ""


# ---- Ghidra discovery -----------------------------------------------------

KNOWN_GHIDRA_DIRS = ("/opt/ghidra", "/usr/local/ghidra", "/usr/share/ghidra")


def discover_ghidra(ghidra_home: str | None = None) -> dict[str, Any]:
    candidates: list[Path] = []
    for env in (ghidra_home, os.environ.get("GHIDRA_HOME"),
                os.environ.get("GHIDRA_INSTALL_DIR")):
        if env:
            candidates.append(Path(env))
    for tool in ("analyzeHeadless", "ghidraRun", "ghidra"):
        found = shutil.which(tool)
        if found:
            # support/analyzeHeadless -> home is two levels up
            p = Path(found).resolve()
            if p.name == "analyzeHeadless":
                candidates.append(p.parent.parent)
            else:
                candidates.append(p.parent)
    candidates.extend(Path(d) for d in KNOWN_GHIDRA_DIRS)

    for home in candidates:
        headless = home / "support" / "analyzeHeadless"
        props = home / "Ghidra" / "application.properties"
        if headless.is_file() and props.is_file():
            version = _read_ghidra_version(props)
            return {
                "found": True,
                "home": str(home),
                "analyze_headless": str(headless),
                "version": version,
                "pyghidra_run": (home / "support" / "pyghidraRun").is_file(),
            }
    return {"found": False, "home": None, "analyze_headless": None,
            "version": None, "pyghidra_run": False}


def _read_ghidra_version(props: Path) -> str | None:
    try:
        for line in props.read_text().splitlines():
            if line.startswith("application.version="):
                return line.split("=", 1)[1].strip()
    except OSError:
        pass
    return None


# ---- Java -----------------------------------------------------------------

def check_java(min_major: int = 21) -> dict[str, Any]:
    java = shutil.which("java")
    if not java:
        return {"found": False, "major": None, "ok": False}
    rc, out = _run([java, "-version"])
    m = re.search(r'version "?(\d+)', out)
    major = int(m.group(1)) if m else None
    return {"found": True, "path": java, "major": major,
            "ok": major is not None and major >= min_major}


# ---- Python / PyGhidra / binutils / frida --------------------------------

def check_python() -> dict[str, Any]:
    import sys
    return {"found": True, "path": sys.executable,
            "version": platform.python_version(),
            "pip": _pip_available()}


def _pip_available() -> bool:
    rc, _ = _run(["python3", "-m", "pip", "--version"])
    if rc == 0:
        return True
    import sys
    rc, _ = _run([sys.executable, "-m", "pip", "--version"])
    return rc == 0


def check_pyghidra(ghidra: dict[str, Any]) -> dict[str, Any]:
    """PyGhidra readiness: distinguish launcher-present from library-importable."""
    launcher = bool(ghidra.get("pyghidra_run"))
    rc, _ = _run(["python3", "-m", "pip", "show", "pyghidra"])
    library = rc == 0
    return {
        "launcher": launcher,
        "library": library,
        "ready": launcher and library,
    }


def check_binutils() -> dict[str, Any]:
    tools = ("file", "readelf", "objdump", "nm", "strings", "ar")
    present = {t: bool(shutil.which(t)) for t in tools}
    return {"tools": present, "ok": all(present.values())}


def check_compilers() -> dict[str, Any]:
    comp = {c: bool(shutil.which(c)) for c in ("gcc", "clang", "clang++", "g++")}
    return {"compilers": comp, "ok": any(comp.values())}


def check_frida() -> dict[str, Any]:
    cli = shutil.which("frida")
    version = None
    if cli:
        rc, out = _run([cli, "--version"])
        version = out.strip() if rc == 0 else None
    rc, _ = _run(["python3", "-m", "pip", "show", "frida"])
    return {"cli": bool(cli), "version": version, "python_module": rc == 0,
            "found": bool(cli) or rc == 0}


# ---- isolation controls -------------------------------------------------
#
# doctor is OBSERVATIONAL and non-mutating: it never pulls an image or starts a
# container. It reports two separate signals: (1) isolation_supported -- the
# runtime's `run` help exposes each mandatory flag (syntax only), and (2)
# dynamic_ready -- a preconfigured, verified execution profile attests every
# mandatory control. Help support alone never sets dynamic_ready.

# maps a control name to a flag token that must appear in `<runtime> run --help`
_RUN_FLAG_FOR = {
    "network_none": "--network",
    "read_only_rootfs": "--read-only",
    "non_root_user": "--user",
    "cap_drop_all": "--cap-drop",
    "seccomp_no_new_privs": "--security-opt",
    "pids_limit": "--pids-limit",
    "memory_limit": "--memory",
}


def check_isolation(profile: dict[str, Any] | None = None) -> dict[str, Any]:
    """Report isolation runtime support and dynamic readiness.

    Two distinct signals:
      - isolation_supported: the detected runtime's `run` help exposes every
        mandatory flag. This is a STATIC capability check (syntax only), never
        proof of enforcement.
      - dynamic_ready: true ONLY when a preconfigured, verified execution
        profile attests every mandatory control (network_none, read-only target,
        non-root, cap-drop, seccomp/no-new-privs, cpu/mem/pid/disk quotas, hard
        timeout, clean env, manifest, teardown). Absent such a profile it is
        false. doctor never executes a container to decide this.
    """
    runtime = None
    for r in ("docker", "podman"):
        if shutil.which(r):
            runtime = r
            break
    vm = any(shutil.which(v) for v in ("qemu-system-x86_64", "firecracker"))

    run_help = ""
    if runtime:
        rc, out = _run([runtime, "run", "--help"])
        if rc == 0:
            run_help = out

    supported: dict[str, bool] = {"container_runtime": bool(runtime), "vm_runtime": vm}
    for control, flag in _RUN_FLAG_FOR.items():
        supported[control] = bool(runtime) and flag in run_help

    support_required = ("container_runtime", "network_none", "read_only_rootfs",
                        "non_root_user", "cap_drop_all", "seccomp_no_new_privs",
                        "pids_limit", "memory_limit")
    isolation_supported = all(supported[k] for k in support_required)
    unsupported = [k for k in support_required if not supported[k]]

    # Every control that a verified profile must attest for dynamic_ready.
    profile_required = ("network_none", "read_only_target", "dedicated_output",
                        "non_root_user", "cap_drop_all", "seccomp",
                        "cpu_limit", "memory_limit", "pids_limit", "disk_quota",
                        "hard_timeout", "clean_env", "manifest", "teardown")
    profile_result = _validate_isolation_profile(profile, profile_required)
    dynamic_ready = isolation_supported and profile_result["complete"]

    return {
        "runtime": runtime,
        "supported": supported,
        "support_required": list(support_required),
        "isolation_supported": isolation_supported,
        "unsupported": unsupported,
        "profile_required": list(profile_required),
        "profile": profile_result,
        "dynamic_ready": dynamic_ready,
        "note": ("help-flag support is syntax only; dynamic_ready requires a "
                 "verified execution profile attesting every control"),
    }


def _validate_isolation_profile(profile: dict[str, Any] | None,
                                required: tuple[str, ...]) -> dict[str, Any]:
    """Validate a GENERATED isolation profile artifact, not self-asserted flags.

    A profile only counts toward dynamic_ready when it is a structured artifact:
      - schema_version == 1 and kind == "isolation-profile"
      - runtime naming the container/VM runtime
      - config_path + config_sha256 (64 hex) pointing at an immutable run config
      - verifier: {tool, evidence} recording who attested it and how
      - controls: a mapping asserting every required control
    A bare dict of booleans is explicitly rejected: assertions are not evidence.
    doctor still does not execute anything; capture re-verifies the actual argv
    and manifest against config_sha256 before any untrusted run.
    """
    if not profile:
        return {"provided": False, "complete": False, "missing": list(required),
                "reason": "no isolation profile provided; untrusted dynamic runs refused"}
    problems: list[str] = []
    if profile.get("schema_version") != 1:
        problems.append("schema_version must be 1")
    if profile.get("kind") != "isolation-profile":
        problems.append("kind must be 'isolation-profile'")
    if not isinstance(profile.get("runtime"), str) or not profile.get("runtime"):
        problems.append("runtime required")
    cfg = profile.get("config_path")
    if not isinstance(cfg, str) or not cfg:
        problems.append("config_path required")
    sha = profile.get("config_sha256")
    if not isinstance(sha, str) or not re.fullmatch(r"[0-9a-f]{64}", sha or ""):
        problems.append("config_sha256 must be 64 hex chars")
    verifier = profile.get("verifier")
    if not isinstance(verifier, dict) or not verifier.get("tool") or not verifier.get("evidence"):
        problems.append("verifier.tool and verifier.evidence required")
    controls = profile.get("controls")
    missing = list(required)
    if isinstance(controls, dict):
        missing = [k for k in required if not controls.get(k)]
        if missing:
            problems.append(f"controls missing/false: {missing}")
    else:
        problems.append("controls mapping required")
    return {"provided": True, "complete": not problems,
            "missing": missing, "problems": problems}


# ---- guidance -------------------------------------------------------------

def _guidance(name: str, os_kind: str, pkgs: list[str], official_url: str,
              impact: str, pkg_cmds: dict[str, str], manual: str,
              verify: str) -> dict[str, Any]:
    cmds = [pkg_cmds[p] for p in pkgs if p in pkg_cmds]
    return {
        "dependency": name,
        "impact": impact,
        "official_url": official_url,
        "install_commands": cmds,        # only detected managers
        "manual_route": manual,          # always present
        "verify_command": verify,
    }


def guidance_ghidra(os_kind: str, pkgs: list[str]) -> dict[str, Any]:
    return _guidance(
        "Ghidra 12.x + JDK 21", os_kind, pkgs,
        "https://github.com/NationalSecurityAgency/ghidra/releases",
        "analyze/decompile/baseline/scripts are external-required without Ghidra",
        {"brew": "brew install --cask ghidra",
         "winget": "winget install --id GhidraSRE.Ghidra",
         "choco": "choco install ghidra"},
        "Download the official ghidra_<version>_PUBLIC ZIP from the releases "
        "page, extract it, install a 64-bit JDK 21, and set GHIDRA_HOME to the "
        "extracted dir (which must contain support/analyzeHeadless).",
        "ghidra doctor --format json")


def guidance_jdk(os_kind: str, pkgs: list[str]) -> dict[str, Any]:
    return _guidance(
        "JDK 21+ (64-bit)", os_kind, pkgs,
        "https://adoptium.net/temurin/releases/?version=21",
        "Ghidra requires a 64-bit JDK 21+ to run analyzeHeadless",
        {"apt-get": "sudo apt-get install -y openjdk-21-jdk",
         "dnf": "sudo dnf install -y java-21-openjdk-devel",
         "pacman": "sudo pacman -S --needed jdk21-openjdk",
         "brew": "brew install openjdk@21",
         "winget": "winget install --id EclipseAdoptium.Temurin.21.JDK",
         "choco": "choco install temurin21"},
        "Install a 64-bit JDK 21 from Adoptium and ensure `java -version` "
        "reports 21 or newer.",
        "java -version")


def guidance_python_venv(os_kind: str, pkgs: list[str]) -> dict[str, Any]:
    return _guidance(
        "Python 3.12+ / venv / pip", os_kind, pkgs,
        "https://www.python.org/downloads/",
        "the CLI needs Python 3.12+; venv/pip needed for optional Frida/PyGhidra",
        {"apt-get": "sudo apt-get install -y python3 python3-venv python3-pip",
         "dnf": "sudo dnf install -y python3 python3-pip",
         "pacman": "sudo pacman -S --needed python python-pip",
         "brew": "brew install python@3.12",
         "winget": "winget install --id Python.Python.3.12",
         "choco": "choco install python312"},
        "Install Python 3.12+ from python.org; create a venv with "
        "`python3 -m venv .venv && . .venv/bin/activate`.",
        "python3 --version")


def guidance_binutils(os_kind: str, pkgs: list[str]) -> dict[str, Any]:
    return _guidance(
        "binutils + file", os_kind, pkgs,
        "https://www.gnu.org/software/binutils/",
        "inspect degrades without file/readelf/objdump/nm/strings/ar",
        {"apt-get": "sudo apt-get install -y binutils file",
         "dnf": "sudo dnf install -y binutils file",
         "pacman": "sudo pacman -S --needed binutils file",
         "brew": "brew install binutils",
         "winget": "winget install --id GnuWin32.Binutils",
         "choco": "choco install binutils"},
        "Install GNU binutils and the `file` utility from your OS packages.",
        "readelf --version")


def guidance_frida(os_kind: str, pkgs: list[str]) -> dict[str, Any]:
    note = ("On Linux, set /proc/sys/kernel/yama/ptrace_scope to 0 (or use "
            "sudo) to allow attach.") if os_kind == "linux" else ""
    return {
        "dependency": "Frida (frida-tools)",
        "impact": "frida capture/trace are external-required without Frida",
        "official_url": "https://frida.re/docs/installation/",
        "install_commands": ["python -m pip install frida-tools"],
        "manual_route": ("Create/activate a venv if Python is externally "
                         "managed, then `python -m pip install frida-tools`."),
        "verify_command": "frida --version",
        "platform_note": note,
    }


def guidance_pyghidra(ghidra: dict[str, Any]) -> dict[str, Any]:
    home = ghidra.get("home") or "<Ghidra>"
    return {
        "dependency": "PyGhidra",
        "impact": "`script run --language python` is external-required without PyGhidra",
        "official_url": "https://github.com/NationalSecurityAgency/ghidra/tree/master/Ghidra/Features/PyGhidra",
        "install_commands": [
            "python -m pip install pyghidra",
            f"python -m pip install --no-index -f {home}/Ghidra/Features/PyGhidra/pypkg/dist pyghidra",
        ],
        "manual_route": ("Prefer a venv. Install pyghidra online, or offline "
                         "from the bundled dist in the Ghidra release. Set "
                         "GHIDRA_INSTALL_DIR to the Ghidra home."),
        "verify_command": "python -m pip show pyghidra",
    }


# ---- top-level ------------------------------------------------------------

def run_doctor(ghidra_home: str | None = None,
               isolation_profile: dict[str, Any] | None = None) -> dict[str, Any]:
    os_kind = _os_kind()
    pkgs = _detect_pkg_managers(os_kind)
    ghidra = discover_ghidra(ghidra_home)
    java = check_java()
    python = check_python()
    pyghidra = check_pyghidra(ghidra)
    binutils = check_binutils()
    compilers = check_compilers()
    frida = check_frida()
    isolation = check_isolation(isolation_profile)

    guidance: list[dict[str, Any]] = []
    if not ghidra["found"]:
        guidance.append(guidance_ghidra(os_kind, pkgs))
    if not java["ok"]:
        guidance.append(guidance_jdk(os_kind, pkgs))
    if not python["pip"]:
        guidance.append(guidance_python_venv(os_kind, pkgs))
    if not binutils["ok"]:
        guidance.append(guidance_binutils(os_kind, pkgs))
    if not frida["found"]:
        guidance.append(guidance_frida(os_kind, pkgs))
    if not pyghidra["ready"]:
        guidance.append(guidance_pyghidra(ghidra))

    return {
        "os": os_kind,
        "package_managers": pkgs,
        "ghidra": ghidra,
        "java": java,
        "python": python,
        "pyghidra": pyghidra,
        "binutils": binutils,
        "compilers": compilers,
        "frida": frida,
        "isolation": isolation,
        "dynamic_ready": isolation["dynamic_ready"],
        "static_ready": bool(ghidra["found"] and java["ok"]),
        "guidance": guidance,
    }
