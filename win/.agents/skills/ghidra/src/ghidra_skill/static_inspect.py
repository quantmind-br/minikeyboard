"""Static binary/archive inspection using native tools (no target execution).

Uses file, readelf/objdump, nm, strings, ar, and hashing. Archives (.a) get a
member inventory; invalid members are recorded as warnings, never dropped.
"""
from __future__ import annotations

import hashlib
import os
import re
import shutil
import subprocess
from pathlib import Path
from typing import Any


def _run(cmd: list[str], timeout: float = 30.0) -> tuple[int, str]:
    # Force the C locale so tool output (readelf/nm/file) is stable English and
    # parseable regardless of the host's LANG/LC_ALL.
    env = dict(os.environ, LC_ALL="C", LANG="C")
    try:
        p = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout, env=env)
        return p.returncode, p.stdout + p.stderr
    except (OSError, subprocess.SubprocessError):
        return 127, ""


def _tool_version(tool: str) -> str | None:
    if not shutil.which(tool):
        return None
    rc, out = _run([tool, "--version"])
    if rc != 0:
        return None
    return out.splitlines()[0].strip() if out.strip() else None


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def detect_kind(path: Path) -> str:
    """Coarse kind from magic bytes: elf | pe | macho | archive | unknown."""
    try:
        head = path.read_bytes()[:8]
    except OSError:
        return "unknown"
    if head[:4] == b"\x7fELF":
        return "elf"
    if head[:2] == b"MZ":
        return "pe"
    if head[:4] in (b"\xfe\xed\xfa\xce", b"\xfe\xed\xfa\xcf",
                    b"\xce\xfa\xed\xfe", b"\xcf\xfa\xed\xfe",
                    b"\xca\xfe\xba\xbe"):
        return "macho"
    if head[:8] == b"!<arch>\n":
        return "archive"
    return "unknown"


def inspect(path: Path, max_strings: int = 200) -> dict[str, Any]:
    path = Path(path)
    if not path.is_file():
        raise FileNotFoundError(str(path))

    warnings: list[str] = []
    kind = detect_kind(path)
    tools = {t: _tool_version(t) for t in ("file", "readelf", "objdump", "nm", "strings", "ar")}

    result: dict[str, Any] = {
        "path": str(path.resolve()),
        "sha256": sha256(path),
        "size": path.stat().st_size,
        "kind": kind,
        "tool_versions": tools,
        "format": None,
        "arch": None,
        "endian": None,
        "sections": [],
        "exports": [],
        "imports": [],
        "archive_members": [],
        "warnings": warnings,
    }

    # `file` description
    if tools["file"]:
        rc, out = _run(["file", "-b", str(path)])
        if rc == 0:
            result["file_description"] = out.strip()

    if kind == "archive":
        result.update(_inspect_archive(path, warnings))
        return result

    if kind == "elf" and tools["readelf"]:
        result.update(_inspect_elf(path, warnings))
    elif kind == "unknown":
        warnings.append("unrecognized magic; not ELF/PE/Mach-O/archive")

    if tools["strings"]:
        rc, out = _run(["strings", "-n", "6", str(path)])
        if rc == 0:
            result["strings_sample"] = out.splitlines()[:max_strings]

    return result


def _inspect_elf(path: Path, warnings: list[str]) -> dict[str, Any]:
    out: dict[str, Any] = {"format": "elf"}
    rc, hdr = _run(["readelf", "-h", str(path)])
    if rc == 0:
        m = re.search(r"Machine:\s+(.+)", hdr)
        if m:
            out["arch"] = m.group(1).strip()
        m = re.search(r"Data:\s+.*(little|big) endian", hdr)
        if m:
            out["endian"] = m.group(1)
        m = re.search(r"Class:\s+(ELF\d+)", hdr)
        if m:
            out["elf_class"] = m.group(1)
    else:
        warnings.append("readelf -h failed")

    rc, secs = _run(["readelf", "-S", "-W", str(path)])
    if rc == 0:
        names = re.findall(r"\]\s+(\.[\w.\-]+)", secs)
        out["sections"] = names
    rc, dyn = _run(["readelf", "-d", "-W", str(path)])
    if rc == 0:
        out["imports"] = re.findall(r"\(NEEDED\).*\[(.+?)\]", dyn)
    if shutil.which("nm"):
        rc, syms = _run(["nm", "-D", "--defined-only", str(path)])
        if rc == 0:
            out["exports"] = [ln.split()[-1] for ln in syms.splitlines() if ln.strip()][:500]
    return out


def _inspect_archive(path: Path, warnings: list[str]) -> dict[str, Any]:
    members: list[dict[str, Any]] = []
    if not shutil.which("ar"):
        warnings.append("ar not available; cannot inventory archive members")
        return {"format": "archive", "archive_members": members}
    rc, out = _run(["ar", "t", str(path)])
    if rc != 0:
        warnings.append("ar t failed; archive may be malformed")
        return {"format": "archive", "archive_members": members}
    names = [ln.strip() for ln in out.splitlines() if ln.strip()]
    seen: dict[str, int] = {}
    for name in names:
        ordinal = seen.get(name, 0)
        seen[name] = ordinal + 1
        member: dict[str, Any] = {"name": name, "ordinal": ordinal, "valid": True, "symbols": []}
        if ordinal > 0:
            member["warning"] = "duplicate member name; symbols not independently resolvable"
            member["valid"] = False
            warnings.append(f"archive has duplicate member name {name!r} (occurrence {ordinal + 1}); skipped symbol extraction")
            members.append(member)
            continue
        if not _safe_member_name(name):
            member["valid"] = False
            member["warning"] = "unsafe member name (absolute or traversal); not extracted"
            warnings.append(f"archive member {name!r} has an unsafe name; skipped extraction")
            members.append(member)
            continue
        syms = _member_symbols(path, name)
        if syms is None:
            member["valid"] = False
            member["warning"] = "could not read member symbols (invalid or unsupported)"
            warnings.append(f"archive member {name!r} appears invalid")
        else:
            member["symbols"] = syms[:200]
        members.append(member)
    return {"format": "archive", "archive_members": members}


def _safe_member_name(name: str) -> bool:
    """Reject absolute paths and traversal in archive member names."""
    from pathlib import PurePosixPath
    if not name or name.startswith("/") or "\\" in name:
        return False
    parts = PurePosixPath(name).parts
    return not any(p in ("..", "") for p in parts) and not PurePosixPath(name).is_absolute()


def _member_symbols(path: Path, name: str) -> list[str] | None:
    """Extract a single member into an isolated temp dir and read its defined
    symbols. Returns None if the member is invalid/unreadable. The member name
    is validated by the caller; extraction is confined to the temp dir and the
    resulting path is verified to stay inside it."""
    import tempfile
    if not shutil.which("nm"):
        return []
    with tempfile.TemporaryDirectory() as td:
        tdp = Path(td).resolve()
        if _ar_supports_output():
            ex_rc, _ = _run(["ar", "x", "--output", td, str(path.resolve()), name])
        else:
            ex_rc, _ = _extract_member_fallback(path, name, td)
        if ex_rc != 0:
            return None
        member_path = (tdp / name).resolve()
        # defense in depth: ensure extraction stayed within the temp dir
        if tdp not in member_path.parents and member_path != tdp:
            return None
        if not member_path.is_file():
            return None
        rc, syms = _run(["nm", "--defined-only", str(member_path)])
        if rc != 0:
            return None
        return [ln.split()[-1] for ln in syms.splitlines() if ln.strip()]


def _ar_supports_output() -> bool:
    rc, out = _run(["ar", "--help"])
    return "--output" in out


def _extract_member_fallback(path: Path, name: str, td: str) -> tuple[int, str]:
    # Older ar extracts into cwd; run with cwd=td so extraction is confined.
    try:
        p = subprocess.run(["ar", "x", str(path.resolve()), name],
                           cwd=td, capture_output=True, text=True, timeout=30)
        return p.returncode, p.stdout + p.stderr
    except (OSError, subprocess.SubprocessError):
        return 127, ""
