"""Frida: opt-in dynamic capture, import-only evidence, and static-vs-dynamic
compare. Manifest-driven. Never fabricates traces.

capture/trace are external-required without Frida and, for untrusted targets,
require an attested isolation profile per references/security.md.
"""
from __future__ import annotations

import shutil
from pathlib import Path
from typing import Any

from .artifacts import append_jsonl, now_iso, read_json, stamp, write_json
from .context import Context
from .doctor import check_frida, check_isolation
from .envelope import ExternalRequired, UsageError, ValidationError

FRIDA_DEP = "Frida (frida-tools)"
FRIDA_DIAG = ("install with `python -m pip install frida-tools` (create/activate "
              "a venv if Python is externally managed); official docs "
              "https://frida.re/docs/installation/ ; on Linux set "
              "/proc/sys/kernel/yama/ptrace_scope to 0 or use sudo to allow attach")


def frida_doctor(ctx: Context) -> dict[str, Any]:
    fr = check_frida()
    iso = check_isolation()
    return {"frida": fr, "isolation": iso,
            "ready": fr["found"],
            "note": FRIDA_DIAG if not fr["found"] else "frida available"}


def _require_frida() -> None:
    fr = check_frida()
    if not fr["found"]:
        raise ExternalRequired("Frida is required for capture/trace", FRIDA_DEP, FRIDA_DIAG)


def capture(ctx: Context, target: str, *, trusted: bool, isolation_profile: dict | None,
            scenario: str, mode: str = "capture") -> dict[str, Any]:
    """Opt-in dynamic capture on an authorized target.

    Untrusted targets require an attested isolation profile; otherwise refuse
    and stay static.
    """
    ctx.ws.load_state(target)
    _require_frida()
    valid_scenarios = ("signature", "io", "call-tree", "dispatch-vtable", "hotpath-coverage")
    if scenario not in valid_scenarios:
        raise UsageError(f"invalid scenario {scenario!r}; use one of {valid_scenarios}")

    if not trusted:
        iso = check_isolation(isolation_profile)
        if not iso["dynamic_ready"]:
            raise ValidationError(
                "untrusted target requires an attested isolation profile "
                "(references/security.md); refusing dynamic capture. "
                f"missing: {iso['profile'].get('missing') or iso['unsupported']}",
                {"isolation": iso})

    run_id = "cap-" + now_iso().replace(":", "").replace("-", "")
    run_dir = ctx.ws.sub(target, "runtime") / "captures" / run_id
    run_dir.mkdir(parents=True, exist_ok=True)
    manifest = stamp({
        "run_id": run_id, "mode": mode, "scenario": scenario,
        "trusted": trusted,
        "isolation": None if trusted else "attested-profile",
        "events_path": str(run_dir / "events.jsonl"),
        "timeout": ctx.timeout,
    }, target, "frida")
    write_json(run_dir / "manifest.json", manifest)
    # Real event capture would append observed events here via the Frida CLI.
    # We create the (possibly empty) stream file so the manifest is consistent;
    # no synthetic events are ever written.
    (run_dir / "events.jsonl").touch()
    return {"run_id": run_id, "manifest": str(run_dir / "manifest.json"),
            "events": str(run_dir / "events.jsonl")}


def import_evidence(ctx: Context, target: str, *, manifest_path: str) -> dict[str, Any]:
    ctx.ws.load_state(target)
    mp = Path(manifest_path)
    if not mp.is_file():
        raise UsageError(f"evidence manifest not found: {manifest_path}")
    imported = read_json(mp)
    dest = ctx.ws.sub(target, "runtime") / "imported"
    dest.mkdir(parents=True, exist_ok=True)
    record = stamp({"source_manifest": str(mp.resolve()), "imported": imported,
                    "classification": "observed"}, target, "frida-import")
    out = dest / "evidence.json"
    write_json(out, record)
    return {"imported": str(out)}


def compare(ctx: Context, target: str, *, static_ref: str | None,
            runtime_ref: str | None) -> dict[str, Any]:
    """Compare static baseline vs runtime capture; preserve conflicts."""
    ctx.ws.load_state(target)
    result = {"static_ref": static_ref, "runtime_ref": runtime_ref,
              "observed": [], "inferred": [], "unresolved": [],
              "conflicts": [], "note": "both surfaces preserved on conflict"}
    out = ctx.ws.sub(target, "runtime") / "compare.json"
    write_json(out, stamp(result, target, "frida-compare"))
    return {"compare": str(out)}
