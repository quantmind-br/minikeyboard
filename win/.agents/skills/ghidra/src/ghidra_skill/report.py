"""Report, progressive compare, and improvement review.

Reports separate observed / inferred / unresolved and reflect recorded artifacts
only. Progressive compare requires explicit target/reason/question/boundary/
fallback/compare inputs. improve review never self-edits the skill.
"""
from __future__ import annotations

from pathlib import Path
from typing import Any

from .artifacts import now_iso, read_json, stamp, write_json
from .context import Context
from .envelope import UsageError


def compare(ctx: Context, target: str, *, reason: str, question: str,
            boundary: str, fallback: str, compare_ref: str) -> dict[str, Any]:
    """Progressive decompilation compare; all six inputs required."""
    ctx.ws.load_state(target)
    missing = [k for k, v in {
        "reason": reason, "question": question, "boundary": boundary,
        "fallback": fallback, "compare": compare_ref}.items() if not v]
    if missing:
        raise UsageError(f"progressive compare requires: {missing}")
    run_id = "cmp-" + now_iso().replace(":", "").replace("-", "")
    record = stamp({
        "run_id": run_id, "reason": reason, "question": question,
        "boundary": boundary, "fallback": fallback, "compare": compare_ref,
        "observed": [], "inferred": [], "unresolved": [], "conflicts": [],
    }, target, "compare")
    out = ctx.ws.sub(target, "reports") / f"{run_id}-compare.json"
    write_json(out, record)
    return {"run_id": run_id, "record": str(out)}


def report(ctx: Context, target: str, *, run_id: str | None = None) -> dict[str, Any]:
    state = ctx.ws.load_state(target)
    run_id = run_id or ("rep-" + now_iso().replace(":", "").replace("-", ""))
    rdir = ctx.ws.sub(target, "reports") / run_id
    rdir.mkdir(parents=True, exist_ok=True)

    observed: list[str] = []
    inferred: list[str] = []
    unresolved: list[str] = []

    baseline_dir = ctx.ws.sub(target, "baseline")
    for name in ("functions", "callgraph", "types", "vtables", "constants", "strings", "imports"):
        if (baseline_dir / f"{name}.json").is_file():
            observed.append(f"baseline/{name}.json")
        else:
            unresolved.append(f"baseline/{name} not exported")

    ev = ctx.ws.sub(target, "evidence") / "third-party.json"
    if ev.is_file():
        observed.append("evidence/third-party.json")

    for g in ("renames", "signatures", "types"):
        if (ctx.ws.sub(target, "metadata") / f"{g}.json").is_file():
            inferred.append(f"metadata/{g}.json")

    doc = stamp({
        "target": target, "status": state["status"], "run_id": run_id,
        "observed": observed, "inferred": inferred, "unresolved": unresolved,
    }, target, "report")
    json_path = rdir / "report.json"
    md_path = rdir / "report.md"
    write_json(json_path, doc)
    md_path.write_text(_render_md(doc), encoding="utf-8")
    return {"run_id": run_id, "report_json": str(json_path), "report_md": str(md_path)}


def _render_md(doc: dict[str, Any]) -> str:
    lines = [f"# Analysis report: {doc['target']}", "",
             f"- status: {doc['status']}", f"- run: {doc['run_id']}", ""]
    for section in ("observed", "inferred", "unresolved"):
        lines.append(f"## {section.capitalize()}")
        items = doc.get(section, [])
        lines.extend(f"- {i}" for i in items) if items else lines.append("- (none)")
        lines.append("")
    return "\n".join(lines)


def improve_review(ctx: Context, target: str, *, candidate: str | None = None,
                   classification: str = "deferred", evidence: list[str] | None = None,
                   overlap: str | None = None, destination: str | None = None) -> dict[str, Any]:
    """Record a reusable-improvement candidate. Never self-edits the skill."""
    ctx.ws.load_state(target)
    if classification not in ("accepted", "deferred", "rejected"):
        raise UsageError(f"invalid classification {classification!r}")
    run_id = "imp-" + now_iso().replace(":", "").replace("-", "")
    record = stamp({
        "run_id": run_id, "candidate": candidate, "classification": classification,
        "evidence": evidence or [], "overlap": overlap,
        "proposed_destination": destination,
        "note": "promotion to skill files is a separate, reviewed maintenance change",
    }, target, "improve-review")
    out = ctx.ws.sub(target, "reports") / f"{run_id}-improve.json"
    write_json(out, record)
    return {"run_id": run_id, "classification": classification, "record": str(out)}
