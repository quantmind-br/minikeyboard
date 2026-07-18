"""Decompilation: single function, batch, and strict-order function analysis.

All decompilation runs through Ghidra (external-required). Selectors resolve
uniquely against the baseline; ambiguous selectors fail before running. Batch
preserves partial successes.
"""
from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Any

from .artifacts import now_iso, read_json, stamp, write_json
from .context import Context
from .envelope import UsageError, ValidationError
from .headless import Ghidra
from .query import resolve_unique

FN_ID_RE = re.compile(r"[^0-9a-zA-Z]+")


def _fn_id(func: dict[str, Any]) -> str:
    addr = (func.get("address") or "").lower().replace("0x", "")
    if addr:
        return f"fn_{FN_ID_RE.sub('', addr)}"
    name = func.get("name") or "unknown"
    return f"fn_{FN_ID_RE.sub('_', name)}"


def _decompile_one(ctx: Context, target: str, gh: Ghidra, func: dict[str, Any],
                   run_dir: Path) -> dict[str, Any]:
    fn_id = _fn_id(func)
    out_dir = ctx.ws.sub(target, "decompilation") / "functions" / fn_id
    out_dir.mkdir(parents=True, exist_ok=True)
    log = out_dir / "decompile.log"
    project = ctx.ws.project_dir(target)
    addr = func.get("address")
    manifest = gh.run_headless(
        project, target, process_existing=True, analysis=False,
        post_scripts=[["DecompileFunction.java", str(addr), str(out_dir)]],
        timeout=ctx.timeout, log_path=log)
    source = out_dir / "source.c"
    ok = manifest["returncode"] == 0 and source.is_file()
    record = stamp({
        "fn_id": fn_id,
        "selector_address": addr,
        "function": func,
        "status": "succeeded" if ok else "failed",
        "source_path": str(source) if source.is_file() else None,
        "manifest": manifest,
    }, target, "decompile")
    write_json(out_dir / "record.json", record)
    return {"fn_id": fn_id, "status": record["status"],
            "source": record["source_path"], "record": str(out_dir / "record.json")}


def decompile(ctx: Context, target: str, *, selectors: list[str],
              batch_file: str | None = None) -> dict[str, Any]:
    ctx.ws.load_state(target)
    gh = Ghidra(ctx.ghidra_home)
    gh.require()

    all_selectors = list(selectors)
    if batch_file:
        bf = Path(batch_file)
        if not bf.is_file():
            raise UsageError(f"batch file not found: {batch_file}")
        loaded = read_json(bf)
        items = loaded.get("functions", loaded) if isinstance(loaded, dict) else loaded
        if not isinstance(items, list):
            raise UsageError("batch file must contain a list of selectors under 'functions'")
        all_selectors += [str(x) for x in items]
    if not all_selectors:
        raise UsageError("no function selectors provided (use --function or --batch)")

    # Resolve ALL selectors uniquely up front; ambiguous/unknown fail before running.
    resolved: list[dict[str, Any]] = []
    for sel in all_selectors:
        resolved.append(resolve_unique(ctx, target, sel))  # raises on ambiguity/miss

    run_id = "run-" + now_iso().replace(":", "").replace("-", "")
    results = []
    succeeded = failed = 0
    for func in resolved:
        r = _decompile_one(ctx, target, gh, func, run_dir=Path())
        results.append(r)
        if r["status"] == "succeeded":
            succeeded += 1
        else:
            failed += 1

    summary = {"run_id": run_id, "succeeded": succeeded, "failed": failed,
               "skipped": 0, "total": len(resolved), "results": results}
    if batch_file or len(all_selectors) > 1:
        batch_path = ctx.ws.sub(target, "decompilation") / "batches" / f"{run_id}.json"
        write_json(batch_path, stamp(summary, target, "decompile"))
        summary["batch_record"] = str(batch_path)
    if succeeded and not failed:
        ctx.ws.set_status(target, "decompiled")
    return summary


# ---- strict-order single-function analysis --------------------------------

STEPS = ("types", "constants_strings", "vtables", "identity_signature", "decompile")


def function_analyze(ctx: Context, target: str, selector: str) -> dict[str, Any]:
    ctx.ws.load_state(target)
    gh = Ghidra(ctx.ghidra_home)
    gh.require()
    func = resolve_unique(ctx, target, selector)  # unique or raise
    fn_id = _fn_id(func)
    analysis_dir = ctx.ws.sub(target, "analysis") / "functions" / fn_id
    analysis_dir.mkdir(parents=True, exist_ok=True)

    steps_record: list[dict[str, Any]] = []
    # Steps 1-4 read from exported baseline (types/constants/strings/vtables/
    # functions). Step 5 decompiles. Strict order, no skips.
    for step in STEPS:
        if step == "decompile":
            r = _decompile_one(ctx, target, gh, func, run_dir=Path())
            evidence = {"source": r["source"], "status": r["status"]}
            conclusion = "decompiled" if r["status"] == "succeeded" else "decompile-failed"
        else:
            evidence = _step_evidence(ctx, target, step, func)
            conclusion = "recovered" if evidence.get("items") else "none-found"
        steps_record.append({
            "step": step, "conclusion": conclusion, "evidence": evidence,
            "observed_at": now_iso(),
        })

    doc = stamp({"fn_id": fn_id, "function": func, "order": list(STEPS),
                 "steps": steps_record}, target, "function-analyze")
    steps_path = analysis_dir / "steps.json"
    write_json(steps_path, doc)
    return {"fn_id": fn_id, "steps": [s["step"] for s in steps_record],
            "steps_path": str(steps_path),
            "decompiled": steps_record[-1]["conclusion"] == "decompiled"}


def _step_evidence(ctx: Context, target: str, step: str, func: dict[str, Any]) -> dict[str, Any]:
    from .query import list_baseline
    mapping = {
        "types": "types",
        "constants_strings": "constants",
        "vtables": "vtables",
        "identity_signature": "functions",
    }
    group = mapping[step]
    base_dir = ctx.ws.sub(target, "baseline")
    if not (base_dir / f"{group}.json").is_file():
        return {"items": [], "note": f"baseline {group} not available"}
    data = list_baseline(ctx, target, group)
    return {"group": group, "items": data.get(group, [])[:50]}
