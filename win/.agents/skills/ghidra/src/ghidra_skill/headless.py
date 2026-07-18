"""Ghidra headless supervision: discovery, safe analyzeHeadless assembly, Java
script execution, allowlisted outputs, timeout, and invocation manifests.

Never executes the target. analyzeHeadless imports/analyzes the binary as data.
When Ghidra is absent, callers surface external-required via ExternalRequired.
"""
from __future__ import annotations

import os
import re
import subprocess
from pathlib import Path
from typing import Any

from .artifacts import now_iso, write_json
from .doctor import discover_ghidra
from .envelope import ExternalRequired, SkillError, UsageError

# Java scripts bundled with the skill (clean-room).
SCRIPTS_DIR = Path(__file__).resolve().parent.parent.parent / "scripts" / "ghidra"

GHIDRA_DEP = "Ghidra 12.x (support/analyzeHeadless) + JDK 21+"
GHIDRA_DIAG = ("run `ghidra doctor` for the official download URL and per-OS "
               "install steps; set GHIDRA_HOME to a dir containing "
               "support/analyzeHeadless")


_TOKEN_RE = re.compile(r"[A-Za-z0-9:._+\-]+", re.ASCII)


def _require_token(value: str, what: str) -> None:
    if not _TOKEN_RE.fullmatch(value or ""):
        raise UsageError(f"invalid {what} value: {value!r}")


def _validate_script_name(name: str, roots: list[Path]) -> None:
    """A script must be a bare filename resolving inside an allowlisted root."""
    if not name or "/" in name or "\\" in name or name in (".", ".."):
        raise UsageError(f"invalid script name (must be a bare filename): {name!r}")
    for root in roots:
        candidate = (root / name).resolve()
        if (root in candidate.parents) and candidate.is_file():
            return
    raise UsageError(
        f"script {name!r} not found in any allowlisted script root: "
        f"{[str(r) for r in roots]}")


class Ghidra:
    def __init__(self, ghidra_home: str | None = None):
        self.info = discover_ghidra(ghidra_home)

    def require(self) -> dict[str, Any]:
        if not self.info["found"]:
            raise ExternalRequired(
                "Ghidra is required for this operation but was not found",
                GHIDRA_DEP, GHIDRA_DIAG)
        return self.info

    @property
    def analyze_headless(self) -> str:
        return self.require()["analyze_headless"]

    def run_headless(self, project_dir: Path, project_name: str, *,
                     import_binary: Path | None = None,
                     process_existing: bool = False,
                     process_file: str | None = None,
                     pre_scripts: list[list[str]] | None = None,
                     post_scripts: list[list[str]] | None = None,
                     allowed_script_roots: list[Path] | None = None,
                     analysis: bool = True,
                     processor: str | None = None,
                     cspec: str | None = None,
                     analysis_timeout: int | None = None,
                     overwrite: bool = False,
                     read_only: bool = False,
                     timeout: float = 600.0,
                     log_path: Path | None = None,
                     script_log_path: Path | None = None) -> dict[str, Any]:
        """Assemble and run analyzeHeadless. Returns an invocation manifest.

        Exactly one of import_binary / process_existing must be chosen. Script
        search roots are constrained to an allowlist (the bundled scripts dir
        plus any explicitly allowed roots, typically the target's scripts dir);
        every -preScript/-postScript name must resolve inside one of them. There
        is no raw extra-args passthrough: analysis knobs are explicit and
        validated.
        """
        self.require()
        project_dir = Path(project_dir)
        project_dir.mkdir(parents=True, exist_ok=True)

        argv: list[str] = [self.analyze_headless, str(project_dir), project_name]
        if import_binary is not None:
            argv += ["-import", str(Path(import_binary).resolve())]
        elif process_existing:
            argv += ["-process"] + ([process_file] if process_file else [])
        else:
            raise UsageError("run_headless needs import_binary or process_existing")

        if not analysis:
            argv += ["-noanalysis"]
        if overwrite:
            argv += ["-overwrite"]
        if read_only:
            argv += ["-readOnly"]
        if processor:
            _require_token(processor, "processor")
            argv += ["-processor", processor]
        if cspec:
            _require_token(cspec, "cspec")
            argv += ["-cspec", cspec]
        if analysis_timeout is not None:
            if int(analysis_timeout) <= 0:
                raise UsageError("analysis_timeout must be a positive integer")
            argv += ["-analysisTimeoutPerFile", str(int(analysis_timeout))]

        # Build the allowlist of script roots and validate every script name.
        roots = [SCRIPTS_DIR.resolve()]
        for r in (allowed_script_roots or []):
            roots.append(Path(r).resolve())
        pre_scripts = pre_scripts or []
        post_scripts = post_scripts or []
        for s in (*pre_scripts, *post_scripts):
            _validate_script_name(s[0], roots)

        # -scriptPath is ONE semicolon-joined argument.
        argv += ["-scriptPath", ";".join(str(r) for r in roots)]
        for s in pre_scripts:
            argv += ["-preScript", *[str(a) for a in s]]
        for s in post_scripts:
            argv += ["-postScript", *[str(a) for a in s]]
        if log_path is not None:
            argv += ["-log", str(log_path)]
        if script_log_path is not None:
            argv += ["-scriptlog", str(script_log_path)]

        env = dict(os.environ, LC_ALL="C", LANG="C")
        if self.info.get("home"):
            env.setdefault("GHIDRA_INSTALL_DIR", self.info["home"])

        started = now_iso()
        try:
            proc = subprocess.run(argv, capture_output=True, text=True,
                                  timeout=timeout, env=env)
            timed_out = False
            rc = proc.returncode
            out = proc.stdout + proc.stderr
        except subprocess.TimeoutExpired as e:
            timed_out = True
            rc = -1
            out = ((e.stdout or "") + (e.stderr or "")) if isinstance(e.stdout, str) else ""
        except OSError as e:
            raise SkillError(f"failed to launch analyzeHeadless: {e}")

        if log_path is not None and not Path(log_path).exists():
            Path(log_path).parent.mkdir(parents=True, exist_ok=True)
            Path(log_path).write_text(out, encoding="utf-8")

        return {
            "tool": "analyzeHeadless",
            "ghidra_version": self.info.get("version"),
            "project_dir": str(project_dir),
            "project_name": project_name,
            "argv": argv,
            "started_at": started,
            "finished_at": now_iso(),
            "returncode": rc,
            "timed_out": timed_out,
            "log_path": str(log_path) if log_path else None,
            "stdout_tail": out[-4000:] if out else "",
        }


def analyze(ctx, target: str, *, rebuild: bool = False, processor: str | None = None,
            cspec: str | None = None, analysis_timeout: int | None = None) -> dict:
    """Import + auto-analyze the target and export the seven baselines.

    external-required when Ghidra is absent (Ghidra.require()).
    """
    from .artifacts import stamp, write_json
    state = ctx.ws.load_state(target)
    gh = Ghidra(ctx.ghidra_home)
    gh.require()

    binary = Path(state["binary"]["path"])
    if not binary.is_file():
        raise SkillError(f"binary missing on disk: {binary}")
    project = ctx.ws.project_dir(target)
    baseline_dir = ctx.ws.sub(target, "baseline")
    baseline_dir.mkdir(parents=True, exist_ok=True)
    log = ctx.ws.sub(target, "intake") / "analyze.log"
    script_log = ctx.ws.sub(target, "intake") / "analyze-script.log"

    manifest = gh.run_headless(
        project, target,
        import_binary=binary,
        analysis=True,
        overwrite=rebuild,
        processor=processor, cspec=cspec, analysis_timeout=analysis_timeout,
        post_scripts=[["ExportBaseline.java", str(baseline_dir)]],
        timeout=ctx.timeout, log_path=log, script_log_path=script_log)

    expected = [f"{n}.json" for n in
                ("functions", "callgraph", "types", "vtables", "constants", "strings", "imports")]
    exported = sorted(p.name for p in baseline_dir.glob("*.json"))
    missing = [n for n in expected if n not in exported]
    ok = manifest["returncode"] == 0 and not missing
    write_json(ctx.ws.sub(target, "intake") / "analyze-manifest.json",
               stamp({"manifest": manifest, "exported": exported, "missing": missing},
                     target, "analyze"))
    if ok:
        ctx.ws.set_status(target, "analyzed")
    else:
        ctx.ws.set_status(target, "failed")
    return {"status": "analyzed" if ok else "failed", "manifest": manifest,
            "exported_baselines": exported, "missing_baselines": missing}
