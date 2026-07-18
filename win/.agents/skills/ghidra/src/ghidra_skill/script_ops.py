"""Ghidra script scaffold / lint / run.

Java is the guaranteed default. Python requires doctor-verified PyGhidra.
Scripts live only under <skill>/scripts/ghidra/ or
<workspace>/artifacts/<id>/scripts/. Symlinks and path traversal are rejected.
"""
from __future__ import annotations

import os
from pathlib import Path
from typing import Any

from .artifacts import now_iso, stamp, write_json
from .context import Context
from .doctor import check_pyghidra, discover_ghidra
from .envelope import ExternalRequired, UsageError, ValidationError
from .headless import SCRIPTS_DIR, Ghidra

PYGHIDRA_DEP = "PyGhidra"
PYGHIDRA_DIAG = (
    "python scripts require PyGhidra. Online: `python -m pip install pyghidra`; "
    "offline: `python -m pip install --no-index -f "
    "<Ghidra>/Ghidra/Features/PyGhidra/pypkg/dist pyghidra` (prefer a venv); set "
    "GHIDRA_INSTALL_DIR to the Ghidra home. Verify with `ghidra doctor`.")

JAVA_TEMPLATE = """\
// {name}.java - clean-room Ghidra script scaffold
// @category Analysis
// @keybinding
// @menupath
// @toolbar
import ghidra.app.script.GhidraScript;

public class {name} extends GhidraScript {{
    @Override
    public void run() throws Exception {{
        // TODO: implement. Write outputs only to the provided output directory.
        println("{name}: hello from headless Ghidra");
    }}
}}
"""

PYTHON_TEMPLATE = """\
# {name}.py - clean-room PyGhidra script scaffold
# @category Analysis
# @runtime PyGhidra
# Write outputs only to the provided output directory.
print("{name}: hello from PyGhidra")
"""


def _allowed_roots(ctx: Context, target: str | None) -> list[Path]:
    roots = [SCRIPTS_DIR.resolve()]
    if target:
        roots.append(ctx.ws.sub(target, "scripts").resolve())
    return roots


def _under_allowlist(path: Path, roots: list[Path]) -> bool:
    rp = path.resolve()
    return any(root == rp or root in rp.parents for root in roots)


def _reject_symlink(path: Path) -> None:
    # reject if the path or any parent up to filesystem root is a symlink
    p = path
    while True:
        if p.is_symlink():
            raise ValidationError(f"symlink not allowed in script path: {p}")
        if p.parent == p:
            break
        p = p.parent


def scaffold(ctx: Context, name: str, *, language: str = "java",
             target: str | None = None) -> dict[str, Any]:
    if language not in ("java", "python"):
        raise UsageError(f"invalid language {language!r} (java|python)")
    if not name.isidentifier():
        raise UsageError(f"invalid script name {name!r} (must be an identifier)")
    if language == "python":
        _require_pyghidra(ctx)
    ext = "java" if language == "java" else "py"
    tmpl = JAVA_TEMPLATE if language == "java" else PYTHON_TEMPLATE
    root = (ctx.ws.sub(target, "scripts") if target else SCRIPTS_DIR).resolve()
    root.mkdir(parents=True, exist_ok=True)
    dest = (root / f"{name}.{ext}").resolve()
    if not _under_allowlist(dest, _allowed_roots(ctx, target)):
        raise ValidationError("scaffold destination escapes the script allowlist")
    dest.write_text(tmpl.format(name=name), encoding="utf-8")
    return {"path": str(dest), "language": language}


REQUIRED_JAVA_METADATA = ("@category",)


def lint(ctx: Context, path: str, *, target: str | None = None) -> dict[str, Any]:
    p = Path(path)
    roots = _allowed_roots(ctx, target)
    if not _under_allowlist(p, roots):
        raise ValidationError(
            f"script path is outside the allowlist {[str(r) for r in roots]}: {path}")
    _reject_symlink(p)
    if not p.is_file():
        raise UsageError(f"script not found: {path}")
    text = p.read_text(encoding="utf-8", errors="replace")
    findings: list[str] = []
    if p.suffix == ".java":
        for meta in REQUIRED_JAVA_METADATA:
            if meta not in text:
                findings.append(f"missing metadata tag {meta}")
        if "extends GhidraScript" not in text:
            findings.append("Java script should extend GhidraScript (headless API)")
    elif p.suffix == ".py":
        if "@runtime PyGhidra" not in text and "pyghidra" not in text.lower():
            findings.append("python script should declare @runtime PyGhidra")
    else:
        findings.append(f"unsupported script extension: {p.suffix}")
    return {"path": str(p.resolve()), "ok": not findings, "findings": findings}


def run(ctx: Context, path: str, *, target: str, args: list[str] | None = None,
        language: str = "java") -> dict[str, Any]:
    state = ctx.ws.load_state(target)
    p = Path(path)
    roots = _allowed_roots(ctx, target)
    if not _under_allowlist(p, roots):
        raise ValidationError(
            f"script path is outside the allowlist {[str(r) for r in roots]}: {path}")
    _reject_symlink(p)
    if not p.is_file():
        raise UsageError(f"script not found: {path}")

    lint_result = lint(ctx, str(p), target=target)
    if not lint_result["ok"]:
        raise ValidationError(f"script failed lint: {lint_result['findings']}")

    if language == "python" or p.suffix == ".py":
        _require_pyghidra(ctx)

    gh = Ghidra(ctx.ghidra_home)
    gh.require()
    out_dir = ctx.ws.sub(target, "scripts") / "runs"
    out_dir.mkdir(parents=True, exist_ok=True)
    run_id = "srun-" + now_iso().replace(":", "").replace("-", "")
    log = out_dir / f"{run_id}.log"
    manifest = gh.run_headless(
        ctx.ws.project_dir(target), target, process_existing=True, analysis=False,
        allowed_script_roots=[p.resolve().parent],
        post_scripts=[[p.name, *(args or [])]],
        timeout=ctx.timeout, log_path=log)
    record = stamp({"run_id": run_id, "script": str(p.resolve()),
                    "args": args or [], "manifest": manifest}, target, "script-run")
    write_json(out_dir / f"{run_id}.json", record)
    return {"run_id": run_id, "manifest": manifest, "record": str(out_dir / f"{run_id}.json")}


def _require_pyghidra(ctx: Context) -> None:
    ghidra = discover_ghidra(ctx.ghidra_home)
    py = check_pyghidra(ghidra)
    if not py["ready"]:
        raise ExternalRequired(
            "PyGhidra is required for python scripts but is not ready",
            PYGHIDRA_DEP, PYGHIDRA_DIAG,
            extra={"pyghidra": py})
