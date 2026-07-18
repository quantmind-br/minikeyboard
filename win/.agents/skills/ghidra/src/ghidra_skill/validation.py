"""Gate computation from artifacts. No P0-P6 exposure.

Gates: intake, baseline, evidence, metadata, decompilation. A gate not required
by the requested flow is `not_applicable`, never `passed`. Legacy P0-P6 states
in imported artifacts are translated in the report, not reintroduced as aliases.
"""
from __future__ import annotations

from pathlib import Path
from typing import Any

from .artifacts import stamp, write_json
from .context import Context

GATES = ("intake", "baseline", "evidence", "metadata", "decompilation")

# Map any legacy imported phase state to a public concept for reporting only.
LEGACY_TRANSLATION = {
    "P0": "intake", "P0.5": "intake", "P1": "baseline", "P2": "evidence",
    "P3": "metadata", "P4": "decompilation", "P5": "decompilation",
    "P6": "runtime",
}

BASELINES = ("functions", "callgraph", "types", "vtables", "constants", "strings", "imports")


def _gate_intake(ctx: Context, target: str) -> dict[str, Any]:
    state_ok = ctx.ws.state_path(target).is_file()
    inspection = (ctx.ws.sub(target, "intake") / "inspection.json").is_file()
    status = "passed" if state_ok else "failed"
    return {"status": status, "state": state_ok, "inspection": inspection}


def _gate_baseline(ctx: Context, target: str) -> dict[str, Any]:
    base = ctx.ws.sub(target, "baseline")
    present = [n for n in BASELINES if (base / f"{n}.json").is_file()]
    if not present:
        return {"status": "not_applicable", "present": []}
    status = "passed" if len(present) == len(BASELINES) else "failed"
    return {"status": status, "present": present, "expected": list(BASELINES)}


def _gate_evidence(ctx: Context, target: str) -> dict[str, Any]:
    p = ctx.ws.sub(target, "evidence") / "third-party.json"
    if not p.is_file():
        return {"status": "not_applicable"}
    return {"status": "passed", "record": str(p)}


def _gate_metadata(ctx: Context, target: str) -> dict[str, Any]:
    mdir = ctx.ws.sub(target, "metadata")
    groups = [g for g in ("renames", "signatures", "types") if (mdir / f"{g}.json").is_file()]
    if not groups:
        return {"status": "not_applicable"}
    verify = (mdir / "verify.json").is_file()
    return {"status": "passed" if verify else "failed", "groups": groups, "verified": verify}


def _gate_decompilation(ctx: Context, target: str) -> dict[str, Any]:
    fdir = ctx.ws.sub(target, "decompilation") / "functions"
    if not fdir.is_dir():
        return {"status": "not_applicable"}
    records = list(fdir.glob("*/record.json"))
    if not records:
        return {"status": "not_applicable"}
    from .artifacts import read_json
    ok = sum(1 for r in records if read_json(r).get("status") == "succeeded")
    return {"status": "passed" if ok else "failed",
            "functions": len(records), "succeeded": ok}


def validate(ctx: Context, target: str) -> dict[str, Any]:
    state = ctx.ws.load_state(target)
    gates = {
        "intake": _gate_intake(ctx, target),
        "baseline": _gate_baseline(ctx, target),
        "evidence": _gate_evidence(ctx, target),
        "metadata": _gate_metadata(ctx, target),
        "decompilation": _gate_decompilation(ctx, target),
    }
    legacy_note = None
    if state["status"] in LEGACY_TRANSLATION:
        legacy_note = f"legacy state {state['status']} -> {LEGACY_TRANSLATION[state['status']]}"

    overall = "passed"
    for g in gates.values():
        if g["status"] == "failed":
            overall = "failed"
            break
    doc = stamp({"target": target, "status": state["status"], "overall": overall,
                 "gates": gates, "legacy_translation": legacy_note}, target, "validate")
    out = ctx.ws.sub(target, "gates") / "latest.json"
    write_json(out, doc)
    if overall == "passed":
        ctx.ws.set_status(target, "validated")
    return {"overall": overall, "gates": {k: v["status"] for k, v in gates.items()},
            "record": str(out)}
