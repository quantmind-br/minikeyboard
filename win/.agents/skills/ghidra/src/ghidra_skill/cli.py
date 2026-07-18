"""ghidra CLI: registry, argument parsing, envelope wiring, exit codes.

Every command emits the JSON envelope on stdout; logs go to stderr. Command
paths are declared in COMMANDS so validate_capabilities.py can verify the
CLI<->capabilities mapping and generate help.
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any, Callable

from . import __version__
from .artifacts import read_json, stamp, write_json
from .context import Context
from .envelope import (
    Envelope, SkillError, emit, error, external_required, ok,
    EXIT_OK, EXIT_FAIL, EXIT_USAGE,
)
from .workspace import Workspace


def log(msg: str) -> None:
    print(msg, file=sys.stderr)


# --------------------------------------------------------------------------
# Command registry. Each entry: path -> (handler, adder). `path` is the exact
# public command string used by capabilities.json (space-separated).
# --------------------------------------------------------------------------

COMMAND_PATHS: list[str] = []


def command_paths() -> list[str]:
    """Public command paths, for validate_capabilities.py."""
    if not COMMAND_PATHS:
        _build_parser()
    return sorted(set(COMMAND_PATHS))


# ---- handlers -------------------------------------------------------------

def _ctx(args) -> Context:
    ws = Workspace(Path(getattr(args, "workspace", ".") or "."))
    return Context(
        workspace=ws,
        fmt=getattr(args, "format", "json") or "json",
        target=getattr(args, "target", None),
        timeout=float(getattr(args, "timeout", None) or 600.0),
        lock_timeout=float(getattr(args, "lock_timeout", None) or 30.0),
        no_wait=bool(getattr(args, "no_wait", False)),
        ghidra_home=getattr(args, "ghidra_home", None),
    )


def h_doctor(args) -> Envelope:
    from . import doctor
    profile = None
    if getattr(args, "isolation_profile", None):
        profile = read_json(Path(args.isolation_profile))
    data = doctor.run_doctor(ghidra_home=getattr(args, "ghidra_home", None),
                             isolation_profile=profile)
    msg = "environment checked"
    return ok(msg, data)


def h_init(args) -> Envelope:
    from . import static_inspect
    ctx = _ctx(args)
    entries = list(getattr(args, "entry", None) or [])
    state = ctx.ws.create_target(args.target, args.binary, args.scope, entries)
    # inspect and persist format + inspection.json under the lock
    with ctx.ws.lock(args.target, timeout=ctx.lock_timeout, no_wait=ctx.no_wait):
        insp = static_inspect.inspect(Path(args.binary))
        state = ctx.ws.load_state(args.target)
        state["binary"]["format"] = insp.get("format")
        ctx.ws.save_state(state)
        ip = ctx.ws.sub(args.target, "intake") / "inspection.json"
        write_json(ip, stamp(insp, args.target, "inspect"))
    arts = [_rel(ctx, ctx.ws.state_path(args.target)), _rel(ctx, ip)]
    return ok(f"initialized target {args.target!r}",
              {"target": args.target, "state": state["status"],
               "format": insp.get("format"), "sha256": state["binary"]["sha256"]},
              arts)


def h_inspect(args) -> Envelope:
    from . import static_inspect
    ctx = _ctx(args)
    if getattr(args, "target", None) and not getattr(args, "path", None):
        state = ctx.ws.load_state(args.target)
        target_path = Path(state["binary"]["path"])
    elif getattr(args, "path", None):
        target_path = Path(args.path)
    else:
        raise SkillError("inspect requires a PATH or --target", {"reason": "usage"})
    data = static_inspect.inspect(target_path)
    return ok("inspection complete", data)


def h_config_scope(args) -> Envelope:
    ctx = _ctx(args)
    target = ctx.resolve_target()
    op = args.scope_op
    with ctx.ws.lock(target, timeout=ctx.lock_timeout, no_wait=ctx.no_wait):
        if op == "show":
            data = ctx.ws.scope_show(target)
        elif op == "set":
            data = ctx.ws.scope_set(target, args.mode, list(args.entry or []))
        elif op == "add":
            data = ctx.ws.scope_add(target, args.entry_value)
        elif op == "remove":
            data = ctx.ws.scope_remove(target, args.entry_value)
        else:
            raise SkillError(f"unknown scope op {op!r}", {"reason": "usage"})
    return ok(f"scope {op}", {"scope": data})


def h_analyze(args) -> Envelope:
    from . import headless
    ctx = _ctx(args)
    target = ctx.resolve_target()
    with ctx.ws.lock(target, timeout=ctx.lock_timeout, no_wait=ctx.no_wait):
        data = headless.analyze(
            ctx, target, rebuild=bool(getattr(args, "rebuild", False)),
            processor=getattr(args, "processor", None),
            cspec=getattr(args, "cspec", None),
            analysis_timeout=getattr(args, "analysis_timeout", None))
    return ok(f"analysis {data['status']}", data)


def h_list(args) -> Envelope:
    from . import query
    ctx = _ctx(args)
    target = ctx.resolve_target()
    group = args.group
    if group == "callgraph":
        data = query.list_callgraph(
            ctx, target, callers=getattr(args, "callers", False),
            callees=getattr(args, "callees", False),
            transitive=getattr(args, "transitive", False),
            selector=getattr(args, "selector", None))
    else:
        data = query.list_baseline(ctx, target, group)
    return ok(f"list {group}", data)


def h_show_function(args) -> Envelope:
    from . import query
    ctx = _ctx(args)
    target = ctx.resolve_target()
    data = query.show_function(ctx, target, args.selector)
    return ok("function", data)


def h_metadata(args) -> Envelope:
    from . import metadata
    ctx = _ctx(args)
    target = ctx.resolve_target()
    op = args.meta_op
    force = bool(getattr(args, "force", False))
    with ctx.ws.lock(target, timeout=ctx.lock_timeout, no_wait=ctx.no_wait):
        if op == "rename":
            data = metadata.record_rename(ctx, target, address=args.address,
                                          new_name=args.new_name,
                                          provenance=args.provenance, force=force)
        elif op == "signature":
            data = metadata.record_signature(ctx, target, address=args.address,
                                             signature=args.signature,
                                             provenance=args.provenance, force=force)
        elif op == "types":
            data = metadata.record_types(ctx, target, name=args.name,
                                         definition=args.definition,
                                         provenance=args.provenance, force=force)
        elif op == "apply":
            data = metadata.apply_metadata(ctx, target, force=force)
        else:
            raise SkillError(f"unknown metadata op {op!r}", {"reason": "usage"})
    return ok(f"metadata {op}", data)


def h_decompile(args) -> Envelope:
    from . import decompile
    ctx = _ctx(args)
    target = ctx.resolve_target()
    with ctx.ws.lock(target, timeout=ctx.lock_timeout, no_wait=ctx.no_wait):
        data = decompile.decompile(ctx, target,
                                   selectors=list(getattr(args, "function", None) or []),
                                   batch_file=getattr(args, "batch", None))
    return ok("decompile complete", data)


def h_function_analyze(args) -> Envelope:
    from . import decompile
    ctx = _ctx(args)
    target = ctx.resolve_target()
    with ctx.ws.lock(target, timeout=ctx.lock_timeout, no_wait=ctx.no_wait):
        data = decompile.function_analyze(ctx, target, args.selector)
    return ok("function analysis complete", data)


def h_evidence(args) -> Envelope:
    from . import evidence
    ctx = _ctx(args)
    target = ctx.resolve_target()
    with ctx.ws.lock(target, timeout=ctx.lock_timeout, no_wait=ctx.no_wait):
        if getattr(args, "none", False):
            data = evidence.none_third_party(ctx, target)
        elif getattr(args, "list", False):
            data = evidence.list_third_party(ctx, target)
        else:
            if not args.library:
                raise SkillError("evidence third-party needs --library (or --none/--list)",
                                 {"reason": "usage"})
            data = evidence.add_third_party(
                ctx, target, library=args.library, version=getattr(args, "version", None),
                source_path=getattr(args, "source", None),
                confidence=getattr(args, "confidence", "low"),
                evidence=list(getattr(args, "evidence", None) or []),
                classification=getattr(args, "classification", "observed"))
    return ok("third-party evidence", data)


def h_compare(args) -> Envelope:
    from . import report
    ctx = _ctx(args)
    target = ctx.resolve_target()
    with ctx.ws.lock(target, timeout=ctx.lock_timeout, no_wait=ctx.no_wait):
        data = report.compare(ctx, target, reason=args.reason, question=args.question,
                              boundary=args.boundary, fallback=args.fallback,
                              compare_ref=args.compare)
    return ok("compare complete", data)


def h_report(args) -> Envelope:
    from . import report
    ctx = _ctx(args)
    target = ctx.resolve_target()
    with ctx.ws.lock(target, timeout=ctx.lock_timeout, no_wait=ctx.no_wait):
        data = report.report(ctx, target, run_id=getattr(args, "run_id", None))
    return ok("report generated", data, [data["report_json"], data["report_md"]])


def h_improve_review(args) -> Envelope:
    from . import report
    ctx = _ctx(args)
    target = ctx.resolve_target()
    with ctx.ws.lock(target, timeout=ctx.lock_timeout, no_wait=ctx.no_wait):
        data = report.improve_review(
            ctx, target, candidate=getattr(args, "candidate", None),
            classification=getattr(args, "classification", "deferred"),
            evidence=list(getattr(args, "evidence", None) or []),
            overlap=getattr(args, "overlap", None),
            destination=getattr(args, "destination", None))
    return ok("improvement review recorded", data)


def h_validate(args) -> Envelope:
    from . import validation
    ctx = _ctx(args)
    target = ctx.resolve_target()
    with ctx.ws.lock(target, timeout=ctx.lock_timeout, no_wait=ctx.no_wait):
        data = validation.validate(ctx, target)
    return ok(f"validation {data['overall']}", data)


def h_frida(args) -> Envelope:
    from . import frida
    ctx = _ctx(args)
    op = args.frida_op
    if op == "doctor":
        return ok("frida doctor", frida.frida_doctor(ctx))
    target = ctx.resolve_target()
    with ctx.ws.lock(target, timeout=ctx.lock_timeout, no_wait=ctx.no_wait):
        if op in ("capture", "trace"):
            profile = None
            if getattr(args, "isolation_profile", None):
                profile = read_json(Path(args.isolation_profile))
            data = frida.capture(ctx, target, trusted=bool(getattr(args, "trusted", False)),
                                 isolation_profile=profile, scenario=args.scenario, mode=op)
        elif op == "import-evidence":
            data = frida.import_evidence(ctx, target, manifest_path=args.manifest)
        elif op == "compare":
            data = frida.compare(ctx, target, static_ref=getattr(args, "static_ref", None),
                                 runtime_ref=getattr(args, "runtime_ref", None))
        else:
            raise SkillError(f"unknown frida op {op!r}", {"reason": "usage"})
    return ok(f"frida {op}", data)


def h_script(args) -> Envelope:
    from . import script_ops
    ctx = _ctx(args)
    op = args.script_op
    if op == "scaffold":
        data = script_ops.scaffold(ctx, args.name, language=getattr(args, "language", "java"),
                                   target=getattr(args, "target", None))
        return ok("script scaffolded", data, [data["path"]])
    if op == "lint":
        data = script_ops.lint(ctx, args.path, target=getattr(args, "target", None))
        return ok("script linted", data)
    if op == "run":
        target = ctx.resolve_target()
        with ctx.ws.lock(target, timeout=ctx.lock_timeout, no_wait=ctx.no_wait):
            data = script_ops.run(ctx, args.path, target=target,
                                  args=list(getattr(args, "arg", None) or []),
                                  language=getattr(args, "language", "java"))
        return ok("script run complete", data)
    raise SkillError(f"unknown script op {op!r}", {"reason": "usage"})


# ---- parser ---------------------------------------------------------------

def _add_global(p: argparse.ArgumentParser, *, suppress: bool = False) -> None:
    # On subparsers, defaults are SUPPRESS so a global given BEFORE the
    # subcommand is not overwritten by the subparser's own default. The
    # top-level parser carries the real defaults.
    def d(v):
        return argparse.SUPPRESS if suppress else v
    p.add_argument("--workspace", default=d("."), help="workspace root (default: .)")
    p.add_argument("--target", default=d(None), help="target id")
    p.add_argument("--format", choices=("json", "text"), default=d("json"))
    p.add_argument("--timeout", type=float, default=d(None), help="operation timeout in seconds")
    p.add_argument("--lock-timeout", type=float, default=d(30.0))
    p.add_argument("--no-wait", action="store_true", default=d(False))
    p.add_argument("--ghidra-home", default=d(None), help="path to Ghidra install")


class _JsonArgumentParser(argparse.ArgumentParser):
    """ArgumentParser that emits the JSON envelope on usage errors (exit 2)."""

    # class-level format, set from parsed globals before dispatch
    _fmt = "json"

    def error(self, message):  # noqa: D401
        env = error(f"usage error: {message}", {"reason": "usage",
                    "prog": self.prog}, EXIT_USAGE)
        emit(env, type(self)._fmt)
        raise SystemExit(EXIT_USAGE)


def _build_parser() -> argparse.ArgumentParser:
    COMMAND_PATHS.clear()
    ap = _JsonArgumentParser(prog="ghidra", description="Headless Ghidra reverse engineering")
    ap.add_argument("--version", action="version", version=f"ghidra {__version__}")
    _add_global(ap)
    sub = ap.add_subparsers(dest="cmd", metavar="COMMAND")

    def cmd(name, handler, help=""):
        p = sub.add_parser(name, help=help)
        _add_global(p, suppress=True)
        p.set_defaults(_handler=handler)
        return p

    # doctor
    d = cmd("doctor", h_doctor, "verify environment")
    d.add_argument("--isolation-profile", help="path to a verified isolation profile JSON")
    COMMAND_PATHS.append("doctor")

    # init
    i = cmd("init", h_init, "initialize a target")
    i.add_argument("binary")
    i.add_argument("--scope", choices=("full", "symbols", "addresses"), default="full")
    i.add_argument("--entry", action="append")
    COMMAND_PATHS.append("init")

    # inspect
    ins = cmd("inspect", h_inspect, "inspect a binary/archive")
    ins.add_argument("path", nargs="?")
    COMMAND_PATHS.append("inspect")

    # config scope
    cfg = cmd("config", None, "configuration")
    cfgsub = cfg.add_subparsers(dest="config_group", metavar="GROUP")
    scope = cfgsub.add_parser("scope", help="analysis scope")
    _add_global(scope, suppress=True)
    scope.set_defaults(_handler=h_config_scope)
    scopesub = scope.add_subparsers(dest="scope_op", metavar="OP")
    for opname in ("show", "set", "add", "remove"):
        sp = scopesub.add_parser(opname)
        _add_global(sp, suppress=True)
        sp.set_defaults(_handler=h_config_scope, scope_op=opname)
        if opname == "set":
            sp.add_argument("--mode", choices=("full", "symbols", "addresses"), default="full")
            sp.add_argument("--entry", action="append")
        if opname in ("add", "remove"):
            sp.add_argument("entry_value")
        COMMAND_PATHS.append(f"config scope {opname}")

    # analyze
    an = cmd("analyze", h_analyze, "import + auto-analyze + export baselines")
    an.add_argument("--rebuild", action="store_true")
    an.add_argument("--processor")
    an.add_argument("--cspec")
    an.add_argument("--analysis-timeout", type=int)
    COMMAND_PATHS.append("analyze")

    # list
    ls = cmd("list", h_list, "list baseline groups")
    lssub = ls.add_subparsers(dest="group", metavar="GROUP")
    for g in ("functions", "callgraph", "types", "vtables", "constants", "strings", "imports"):
        gp = lssub.add_parser(g)
        _add_global(gp, suppress=True)
        gp.set_defaults(_handler=h_list, group=g)
        if g == "callgraph":
            gp.add_argument("--callers", action="store_true")
            gp.add_argument("--callees", action="store_true")
            gp.add_argument("--transitive", action="store_true")
            gp.add_argument("--selector")
        COMMAND_PATHS.append(f"list {g}")

    # show function
    sh = cmd("show", None, "show a single item")
    shsub = sh.add_subparsers(dest="show_group", metavar="GROUP")
    shf = shsub.add_parser("function")
    _add_global(shf, suppress=True)
    shf.add_argument("selector")
    shf.set_defaults(_handler=h_show_function)
    COMMAND_PATHS.append("show function")

    # metadata
    md = cmd("metadata", None, "record/apply metadata")
    mdsub = md.add_subparsers(dest="meta_op", metavar="OP")
    for opname in ("rename", "signature", "types", "apply"):
        mp = mdsub.add_parser(opname)
        _add_global(mp, suppress=True)
        mp.set_defaults(_handler=h_metadata, meta_op=opname)
        mp.add_argument("--force", action="store_true")
        if opname == "rename":
            mp.add_argument("--address", required=True)
            mp.add_argument("--new-name", required=True)
            mp.add_argument("--provenance", required=True)
        elif opname == "signature":
            mp.add_argument("--address", required=True)
            mp.add_argument("--signature", required=True)
            mp.add_argument("--provenance", required=True)
        elif opname == "types":
            mp.add_argument("--name", required=True)
            mp.add_argument("--definition", required=True)
            mp.add_argument("--provenance", required=True)
        COMMAND_PATHS.append(f"metadata {opname}")

    # decompile
    dec = cmd("decompile", h_decompile, "decompile function(s)")
    dec.add_argument("--function", action="append")
    dec.add_argument("--batch")
    COMMAND_PATHS.append("decompile")

    # function analyze
    fa = cmd("function", None, "single-function operations")
    fasub = fa.add_subparsers(dest="function_op", metavar="OP")
    faa = fasub.add_parser("analyze")
    _add_global(faa, suppress=True)
    faa.add_argument("selector")
    faa.set_defaults(_handler=h_function_analyze)
    COMMAND_PATHS.append("function analyze")

    # evidence
    ev = cmd("evidence", None, "third-party evidence")
    evsub = ev.add_subparsers(dest="evidence_group", metavar="GROUP")
    tp = evsub.add_parser("third-party")
    _add_global(tp, suppress=True)
    tp.set_defaults(_handler=h_evidence)
    tp.add_argument("--library")
    tp.add_argument("--version")
    tp.add_argument("--source")
    tp.add_argument("--confidence", choices=("low", "medium", "high"), default="low")
    tp.add_argument("--classification", choices=("observed", "inferred", "unresolved"), default="observed")
    tp.add_argument("--evidence", action="append")
    tp.add_argument("--none", action="store_true")
    tp.add_argument("--list", action="store_true")
    COMMAND_PATHS.append("evidence third-party")

    # compare
    cm = cmd("compare", h_compare, "progressive decompilation compare")
    cm.add_argument("--reason", required=True)
    cm.add_argument("--question", required=True)
    cm.add_argument("--boundary", required=True)
    cm.add_argument("--fallback", required=True)
    cm.add_argument("--compare", required=True)
    COMMAND_PATHS.append("compare")

    # report
    rp = cmd("report", h_report, "generate a report")
    rp.add_argument("--run-id")
    COMMAND_PATHS.append("report")

    # improve review
    im = cmd("improve", None, "improvement review")
    imsub = im.add_subparsers(dest="improve_op", metavar="OP")
    imr = imsub.add_parser("review")
    _add_global(imr, suppress=True)
    imr.set_defaults(_handler=h_improve_review)
    imr.add_argument("--candidate")
    imr.add_argument("--classification", choices=("accepted", "deferred", "rejected"), default="deferred")
    imr.add_argument("--evidence", action="append")
    imr.add_argument("--overlap")
    imr.add_argument("--destination")
    COMMAND_PATHS.append("improve review")

    # validate
    cmd("validate", h_validate, "compute gates").set_defaults(_handler=h_validate)
    COMMAND_PATHS.append("validate")

    # frida
    fr = cmd("frida", None, "optional dynamic capture")
    frsub = fr.add_subparsers(dest="frida_op", metavar="OP")
    for opname in ("doctor", "capture", "trace", "compare", "import-evidence"):
        fp = frsub.add_parser(opname)
        _add_global(fp, suppress=True)
        fp.set_defaults(_handler=h_frida, frida_op=opname)
        if opname in ("capture", "trace"):
            fp.add_argument("--scenario", default="io",
                            choices=("signature", "io", "call-tree", "dispatch-vtable", "hotpath-coverage"))
            fp.add_argument("--trusted", action="store_true")
            fp.add_argument("--isolation-profile")
        if opname == "import-evidence":
            fp.add_argument("--manifest", required=True)
        if opname == "compare":
            fp.add_argument("--static-ref")
            fp.add_argument("--runtime-ref")
        COMMAND_PATHS.append(f"frida {opname}")

    # script
    sc = cmd("script", None, "Ghidra script authoring")
    scsub = sc.add_subparsers(dest="script_op", metavar="OP")
    scaf = scsub.add_parser("scaffold")
    _add_global(scaf, suppress=True)
    scaf.set_defaults(_handler=h_script, script_op="scaffold")
    scaf.add_argument("name")
    scaf.add_argument("--language", choices=("java", "python"), default="java")
    lintp = scsub.add_parser("lint")
    _add_global(lintp, suppress=True)
    lintp.set_defaults(_handler=h_script, script_op="lint")
    lintp.add_argument("path")
    runp = scsub.add_parser("run")
    _add_global(runp, suppress=True)
    runp.set_defaults(_handler=h_script, script_op="run")
    runp.add_argument("path")
    runp.add_argument("--arg", action="append")
    runp.add_argument("--language", choices=("java", "python"), default="java")
    for opname in ("scaffold", "lint", "run"):
        COMMAND_PATHS.append(f"script {opname}")

    return ap


def main(argv: list[str] | None = None) -> int:
    ap = _build_parser()
    # Best-effort: pick up --format from raw argv so argparse usage errors emit
    # in the requested format before full parsing succeeds.
    raw = list(sys.argv[1:] if argv is None else argv)
    if "--format" in raw:
        try:
            _JsonArgumentParser._fmt = raw[raw.index("--format") + 1]
        except (IndexError, ValueError):
            pass
    if _JsonArgumentParser._fmt not in ("json", "text"):
        _JsonArgumentParser._fmt = "json"
    args = ap.parse_args(argv)
    fmt = getattr(args, "format", "json") or "json"
    handler = getattr(args, "_handler", None)
    if handler is None:
        ap.print_help(sys.stderr)
        return EXIT_USAGE
    try:
        env = handler(args)
    except SkillError as e:
        return emit(e.to_envelope(), fmt)
    except BrokenPipeError:
        return EXIT_FAIL
    except Exception as e:  # unexpected: report as operational failure
        log(f"unexpected error: {e!r}")
        return emit(error(f"unexpected error: {e}", {"reason": "internal"}, EXIT_FAIL), fmt)
    return emit(env, fmt)


def _rel(ctx: Context, path: Path) -> str:
    from .artifacts import relpath
    return relpath(path, ctx.ws.root)


if __name__ == "__main__":
    raise SystemExit(main())
