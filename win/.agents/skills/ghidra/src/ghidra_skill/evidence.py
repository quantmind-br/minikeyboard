"""Third-party evidence: identify libraries, record pristine sources.

Records sources by path + hash WITHOUT modifying them. Separates observed /
inferred / unresolved. Never fetches sources.
"""
from __future__ import annotations

import hashlib
from pathlib import Path
from typing import Any

from .artifacts import read_json, stamp, write_json
from .context import Context
from .envelope import UsageError


def _evidence_path(ctx: Context, target: str) -> Path:
    return ctx.ws.sub(target, "evidence") / "third-party.json"


def _hash_source(source_path: str | None) -> dict[str, Any] | None:
    if not source_path:
        return None
    p = Path(source_path)
    if not p.exists():
        raise UsageError(f"source path not found: {source_path}")
    if p.is_file():
        h = hashlib.sha256(p.read_bytes()).hexdigest()
        return {"path": str(p.resolve()), "kind": "file", "sha256": h}
    # directory: hash the sorted tree of file digests (pristine, read-only)
    digests = []
    for f in sorted(p.rglob("*")):
        if f.is_file():
            digests.append(f"{f.relative_to(p)}:{hashlib.sha256(f.read_bytes()).hexdigest()}")
    tree = hashlib.sha256("\n".join(digests).encode()).hexdigest()
    return {"path": str(p.resolve()), "kind": "dir", "tree_sha256": tree,
            "file_count": len(digests)}


def add_third_party(ctx: Context, target: str, *, library: str, version: str | None,
                    source_path: str | None, confidence: str,
                    evidence: list[str], classification: str = "observed") -> dict[str, Any]:
    if classification not in ("observed", "inferred", "unresolved"):
        raise UsageError(f"invalid classification: {classification!r}")
    if confidence not in ("low", "medium", "high"):
        raise UsageError(f"invalid confidence: {confidence!r}")
    path = _evidence_path(ctx, target)
    doc = read_json(path) if path.is_file() else stamp(
        {"libraries": []}, target, "evidence")
    entry = {
        "library": library,
        "version": version,
        "confidence": confidence,
        "classification": classification,
        "evidence": list(evidence),
        "pristine": _hash_source(source_path),
    }
    doc.setdefault("libraries", [])
    doc["libraries"].append(entry)
    write_json(path, doc)
    return {"library": library, "entry": entry, "count": len(doc["libraries"])}


def none_third_party(ctx: Context, target: str) -> dict[str, Any]:
    path = _evidence_path(ctx, target)
    doc = stamp({"libraries": [], "reviewed_none": True}, target, "evidence")
    write_json(path, doc)
    return {"libraries": []}


def list_third_party(ctx: Context, target: str) -> dict[str, Any]:
    path = _evidence_path(ctx, target)
    if not path.is_file():
        return {"libraries": [], "reviewed": False}
    doc = read_json(path)
    return {"libraries": doc.get("libraries", []),
            "reviewed_none": doc.get("reviewed_none", False)}
