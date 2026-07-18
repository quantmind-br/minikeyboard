"""Read structured baseline exports. Requires a prior `analyze`.

Callgraph covers DIRECT call edges only (callers/callees, optional transitive).
General cross-references (xrefs) are NOT implemented; see SKILL.md.
"""
from __future__ import annotations

from pathlib import Path
from typing import Any

from .artifacts import read_json
from .context import Context
from .envelope import UsageError, ValidationError

BASELINES = ("functions", "callgraph", "types", "vtables", "constants", "strings", "imports")


def _baseline_path(ctx: Context, target: str, name: str) -> Path:
    return ctx.ws.sub(target, "baseline") / f"{name}.json"


def _load_baseline(ctx: Context, target: str, name: str) -> Any:
    p = _baseline_path(ctx, target, name)
    if not p.is_file():
        raise UsageError(
            f"baseline '{name}' not found for target {target!r}; run `ghidra analyze` first")
    return read_json(p)


def list_baseline(ctx: Context, target: str, name: str) -> dict[str, Any]:
    if name not in BASELINES:
        raise UsageError(f"unknown baseline group: {name!r}")
    data = _load_baseline(ctx, target, name)
    items = data.get(name, data if isinstance(data, list) else [])
    return {"group": name, "count": len(items) if isinstance(items, list) else None,
            name: items}


def list_callgraph(ctx: Context, target: str, *, callers: bool = False,
                   callees: bool = False, transitive: bool = False,
                   selector: str | None = None) -> dict[str, Any]:
    data = _load_baseline(ctx, target, "callgraph")
    edges = data.get("callgraph", data.get("edges", []))
    result: dict[str, Any] = {"group": "callgraph", "mode": "direct-call-edges",
                              "note": "direct call edges only; xrefs not implemented"}
    if not (callers or callees):
        result["edges"] = edges
        result["count"] = len(edges)
        return result
    if not selector:
        raise UsageError("--callers/--callees require a function selector")
    direction = "callers" if callers else "callees"
    result["direction"] = direction
    result["selector"] = selector
    result["transitive"] = transitive
    result["functions"] = _walk_calls(edges, selector, direction, transitive)
    return result


def _walk_calls(edges: list[dict[str, Any]], selector: str, direction: str,
                transitive: bool) -> list[str]:
    # edges are {"caller": <id>, "callee": <id>}
    def neighbors(node: str) -> list[str]:
        if direction == "callees":
            return [e["callee"] for e in edges if e.get("caller") == node]
        return [e["caller"] for e in edges if e.get("callee") == node]

    seen: list[str] = []
    frontier = neighbors(selector)
    visited = set()
    while frontier:
        n = frontier.pop(0)
        if n in visited:
            continue
        visited.add(n)
        seen.append(n)
        if transitive:
            frontier.extend(neighbors(n))
    return seen


def show_function(ctx: Context, target: str, selector: str) -> dict[str, Any]:
    data = _load_baseline(ctx, target, "functions")
    funcs = data.get("functions", data if isinstance(data, list) else [])
    matches = [f for f in funcs
               if selector in (f.get("name"), f.get("address"), f.get("id"))]
    if not matches:
        # partial name help
        near = [f.get("name") for f in funcs if selector and f.get("name") and selector in f["name"]]
        raise UsageError(
            f"no function matches selector {selector!r}"
            + (f"; similar: {near[:5]}" if near else ""))
    if len(matches) > 1:
        raise ValidationError(
            f"selector {selector!r} is ambiguous ({len(matches)} matches); "
            f"use a unique address or id")
    return {"function": matches[0]}


def resolve_unique(ctx: Context, target: str, selector: str) -> dict[str, Any]:
    """Resolve a selector to a unique function record or raise."""
    return show_function(ctx, target, selector)["function"]
