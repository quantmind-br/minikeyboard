#!/usr/bin/env python3
"""Deterministically grade saved agent responses to ghidra-skill prompts.

Contract (local): each evaluation has assertions with any of
`required_all | required_any | ordered | forbidden_any`. A response passes an
assertion when every present constraint holds. Success requires a complete
`new` variant at 100% and no `old -> new` regression per case.

Usage:
  python evals/run_evals.py --manifest evals/evals.json --outputs responses.json --report report.json
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import Path
from typing import Any

FIELDS = ("required_all", "required_any", "ordered", "forbidden_any")


def _terms(value: Any) -> list[str]:
    if value is None:
        return []
    if not isinstance(value, list) or not all(isinstance(v, str) for v in value):
        raise ValueError("assertion terms must be a list of strings")
    return value


def validate_manifest(manifest: dict[str, Any]) -> None:
    if manifest.get("schemaVersion") != 1:
        raise ValueError("schemaVersion must be 1")
    evals = manifest.get("evaluations")
    if not isinstance(evals, list) or not evals:
        raise ValueError("evaluations must be a non-empty list")
    ids: set[str] = set()
    for item in evals:
        if not isinstance(item.get("id"), str) or not item["id"]:
            raise ValueError("each evaluation needs a non-empty id")
        if item["id"] in ids:
            raise ValueError(f"duplicate evaluation id: {item['id']}")
        ids.add(item["id"])
        if not isinstance(item.get("prompt"), str) or not item["prompt"].strip():
            raise ValueError(f"{item['id']}: prompt must be non-empty")
        assertions = item.get("assertions")
        if not isinstance(assertions, list) or not assertions:
            raise ValueError(f"{item['id']}: assertions must be non-empty")
        for a in assertions:
            if not isinstance(a.get("id"), str):
                raise ValueError(f"{item['id']}: assertion needs an id")
            for f in FIELDS:
                _terms(a.get(f))
            if not any(a.get(f) for f in FIELDS):
                raise ValueError(f"{item['id']}/{a['id']}: empty assertion")


def _matched(text: str, terms: list[str]) -> list[str]:
    low = text.casefold()
    return [t for t in terms if t.casefold() in low]


def _forbidden_hits(text: str, terms: list[str]) -> list[str]:
    """Positive prohibited claims only; skip clearly negated occurrences."""
    low = text.casefold()
    hits = []
    for term in terms:
        needle = term.casefold()
        start = 0
        while True:
            i = low.find(needle, start)
            if i < 0:
                break
            before = low[max(0, i - 48):i]
            after = low[i + len(needle):i + len(needle) + 48]
            negated_before = re.search(
                r"(?:\b(?:do not|don't|never|cannot|can't|refuse(?:\s+to)?|avoid|without|no)\b[^.!?;:]{0,24})$",
                before) is not None
            negated_after = re.match(
                r"\s*(?:(?:is|are|was|were)\s+)?(?:unavailable|forbidden|prohibited|disallowed|unsafe|disabled|not\s+(?:implemented|supported|available))\b",
                after) is not None
            if not (negated_before or negated_after):
                hits.append(term)
                break
            start = i + len(needle)
    return hits


def grade_response(item: dict[str, Any], response: str) -> dict[str, Any]:
    results = []
    for a in item["assertions"]:
        req_all = _terms(a.get("required_all"))
        req_any = _terms(a.get("required_any"))
        ordered = _terms(a.get("ordered"))
        forbidden = _terms(a.get("forbidden_any"))
        forbidden_hits = _forbidden_hits(response, forbidden)
        missing = [t for t in req_all if not _matched(response, [t])]
        any_hits = _matched(response, req_any)
        positions = [response.casefold().find(t.casefold()) for t in ordered]
        ordered_ok = all(p >= 0 for p in positions) and positions == sorted(positions)
        if forbidden_hits:
            passed, reason = False, "forbidden evidence"
        elif missing:
            passed, reason = False, "missing required evidence"
        elif req_any and not any_hits:
            passed, reason = False, "missing one-of evidence"
        elif ordered and not ordered_ok:
            passed, reason = False, "required order not observed"
        else:
            passed, reason = True, "matched"
        results.append({"id": a["id"], "passed": passed, "reason": reason,
                        "matched": {"required_all": _matched(response, req_all),
                                    "required_any": any_hits,
                                    "ordered": _matched(response, ordered),
                                    "forbidden_any": forbidden_hits}})
    return {"id": item["id"], "passed": all(r["passed"] for r in results), "assertions": results}


def _response(value: Any) -> tuple[str, Any]:
    if isinstance(value, str):
        return value, None
    if isinstance(value, dict) and isinstance(value.get("response"), str):
        return value["response"], value.get("timing_ms")
    raise ValueError("a response must be a string or {response, timing_ms}")


def evaluate(manifest_path: Path, responses_path: Path, output_path: Path) -> dict[str, Any]:
    manifest = json.loads(Path(manifest_path).read_text())
    validate_manifest(manifest)
    responses = json.loads(Path(responses_path).read_text())
    if not isinstance(responses, dict) or not responses:
        raise ValueError("responses must map variants to prompt responses")
    variants: dict[str, list[dict[str, Any]]] = {}
    summary: dict[str, dict[str, int]] = {}
    for variant, answers in responses.items():
        if not isinstance(answers, dict):
            raise ValueError("responses must map string variants to response maps")
        graded = []
        for item in manifest["evaluations"]:
            answer, timing = _response(answers.get(item["id"], ""))
            row = grade_response(item, answer)
            row["response_sha256"] = hashlib.sha256(answer.encode("utf8")).hexdigest()
            row["timing_ms"] = timing
            graded.append(row)
        variants[variant] = graded
        summary[variant] = {"passed": sum(r["passed"] for r in graded), "total": len(graded)}
    comparison = {}
    if "old" in variants and "new" in variants:
        comparison = {item["id"]: {"old": variants["old"][i]["passed"],
                                   "new": variants["new"][i]["passed"]}
                      for i, item in enumerate(manifest["evaluations"])}
    report = {"schemaVersion": 1, "manifest": str(manifest_path), "variants": variants,
              "summary": summary, "comparison": comparison}
    Path(output_path).parent.mkdir(parents=True, exist_ok=True)
    Path(output_path).write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    return report


def is_success(report: dict[str, Any]) -> bool:
    new = report.get("summary", {}).get("new")
    if not new or new["passed"] != new["total"]:
        return False
    return all(not (row["old"] and not row["new"]) for row in report.get("comparison", {}).values())


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--manifest", required=True)
    ap.add_argument("--outputs", required=True)
    ap.add_argument("--report", required=True)
    args = ap.parse_args()
    report = evaluate(Path(args.manifest), Path(args.outputs), Path(args.report))
    print(json.dumps(report["summary"], indent=2))
    return 0 if is_success(report) else 1


if __name__ == "__main__":
    raise SystemExit(main())
