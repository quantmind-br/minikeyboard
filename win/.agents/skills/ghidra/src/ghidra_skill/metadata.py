"""Metadata recording and application.

`rename|signature|types` RECORD intent with provenance (static-read). `apply`
mutates the Ghidra project (static-ghidra, external-required) then re-exports to
verify. Conflicts refuse to overwrite without --force.
"""
from __future__ import annotations

from pathlib import Path
from typing import Any

from .artifacts import read_json, stamp, write_json
from .context import Context
from .envelope import UsageError, ValidationError
from .headless import Ghidra

GROUPS = ("renames", "signatures", "types")


def _meta_path(ctx: Context, target: str, group: str) -> Path:
    return ctx.ws.sub(target, "metadata") / f"{group}.json"


def _record(ctx: Context, target: str, group: str, entry: dict[str, Any],
            key: str, force: bool) -> dict[str, Any]:
    path = _meta_path(ctx, target, group)
    doc = read_json(path) if path.is_file() else stamp({group: []}, target, "metadata")
    doc.setdefault(group, [])
    existing = [e for e in doc[group] if e.get(key) == entry.get(key)]
    if existing and not force:
        raise ValidationError(
            f"{group} entry for {entry.get(key)!r} already exists; pass --force to overwrite")
    doc[group] = [e for e in doc[group] if e.get(key) != entry.get(key)]
    doc[group].append(entry)
    write_json(path, doc)
    return {"group": group, "entry": entry, "count": len(doc[group])}


def record_rename(ctx: Context, target: str, *, address: str, new_name: str,
                  provenance: str, force: bool = False) -> dict[str, Any]:
    ctx.ws.load_state(target)
    return _record(ctx, target, "renames",
                   {"address": address, "new_name": new_name, "provenance": provenance},
                   key="address", force=force)


def record_signature(ctx: Context, target: str, *, address: str, signature: str,
                     provenance: str, force: bool = False) -> dict[str, Any]:
    ctx.ws.load_state(target)
    return _record(ctx, target, "signatures",
                   {"address": address, "signature": signature, "provenance": provenance},
                   key="address", force=force)


def record_types(ctx: Context, target: str, *, name: str, definition: str,
                 provenance: str, force: bool = False) -> dict[str, Any]:
    ctx.ws.load_state(target)
    return _record(ctx, target, "types",
                   {"name": name, "definition": definition, "provenance": provenance},
                   key="name", force=force)


def apply_metadata(ctx: Context, target: str, *, force: bool = False) -> dict[str, Any]:
    """Apply recorded renames/signatures via Ghidra, then re-export to verify.

    external-required when Ghidra is absent.
    """
    state = ctx.ws.load_state(target)
    gh = Ghidra(ctx.ghidra_home)
    gh.require()  # raises ExternalRequired if missing

    recorded = {g: _meta_path(ctx, target, g) for g in GROUPS}
    present = {g: p for g, p in recorded.items() if p.is_file()}
    if not present:
        raise UsageError(f"no recorded metadata to apply for target {target!r}")

    project = ctx.ws.project_dir(target)
    log = ctx.ws.sub(target, "metadata") / "apply.log"
    apply_records_dir = ctx.ws.sub(target, "metadata") / "apply-records"
    apply_records_dir.mkdir(parents=True, exist_ok=True)

    post = []
    for g, p in present.items():
        post.append(["ApplyMetadata.java", g, str(p), str(apply_records_dir)])
    verify_out = ctx.ws.sub(target, "metadata") / "verify.json"
    post.append(["VerifyMetadata.java", str(apply_records_dir), str(verify_out)])

    manifest = gh.run_headless(
        project, target, process_existing=True, analysis=False,
        post_scripts=post, timeout=ctx.timeout, log_path=log)

    ctx.ws.set_status(target, "enriched")
    return {"applied_groups": list(present), "manifest": manifest,
            "verify_output": str(verify_out)}
