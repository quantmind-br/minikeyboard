# CLI reference

## Global flags

Accepted by every command:

- `--workspace DIR` — workspace root (default `.`).
- `--target ID` — target selector.
- `--format json|text` — output format (default `json`).
- `--timeout SECONDS` — operation timeout.
- `--lock-timeout SECONDS` — lock acquisition timeout (default 30).
- `--no-wait` — do not wait for the lock.
- `--ghidra-home DIR` — Ghidra install path.

## Output envelope

stdout is always a single JSON object (logs go to stderr):

```json
{ "status": "ok|error", "message": "...", "data": {}, "artifacts": [] }
```

`status` is strictly `ok` or `error`. A missing external dependency is an
`error` with `data.reason == "external-required"`, `data.dependency`, and
`data.diagnostic`.

## Exit codes

- `0` success
- `1` operational failure
- `2` usage / validation error AND external-required
- `32` lock acquisition timeout

## Commands

| Command | Purpose |
|---|---|
| `ghidra doctor [--isolation-profile FILE] [--ghidra-home DIR]` | verify environment |
| `ghidra init BINARY --target ID [--scope full\|symbols\|addresses] [--entry V]...` | initialize target |
| `ghidra inspect [PATH] [--target ID]` | static inspection |
| `ghidra config scope show\|set\|add\|remove --target ID` | analysis scope |
| `ghidra analyze --target ID [--rebuild] [--processor X] [--cspec Y] [--analysis-timeout N]` | import + analyze + baseline |
| `ghidra list functions\|callgraph\|types\|vtables\|constants\|strings\|imports --target ID` | read baselines |
| `ghidra show function SELECTOR --target ID` | one function record |
| `ghidra metadata rename\|signature\|types --target ID ... [--force]` | record metadata |
| `ghidra metadata apply --target ID [--force]` | apply + verify metadata |
| `ghidra decompile --target ID [--function N]... [--batch FILE]` | decompile |
| `ghidra function analyze SELECTOR --target ID` | strict 5-step analysis |
| `ghidra evidence third-party --target ID [--library L --version V --source P --confidence C --evidence E]... [--none] [--list]` | third-party evidence |
| `ghidra compare --target ID --reason --question --boundary --fallback --compare` | progressive compare |
| `ghidra report --target ID [--run-id ID]` | report |
| `ghidra improve review --target ID [--candidate --classification --evidence --overlap --destination]` | improvement review |
| `ghidra validate --target ID` | compute gates |
| `ghidra frida doctor\|capture\|trace\|compare\|import-evidence --target ID ...` | optional dynamic |
| `ghidra script scaffold NAME [--language java\|python]` | scaffold script |
| `ghidra script lint PATH` | lint script |
| `ghidra script run PATH --target ID [--arg V]... [--language java\|python]` | run script |

## Callgraph limitation

`ghidra list callgraph --callers|--callees [--transitive] --selector ADDR`
covers **direct call edges only**. General cross-references (xrefs) are not
implemented; the capability is `shipped-partial` and the limitation is declared
in `capabilities.json`.

## Examples

```sh
# triage a binary statically
ghidra doctor
ghidra init ./sample --target s1
ghidra inspect --target s1

# full static analysis
ghidra analyze --target s1
ghidra list functions --target s1
ghidra show function --target s1 main
ghidra decompile --target s1 --function main
ghidra validate --target s1
```

## Common errors

- `target not found` — run `ghidra init` first (exit 2).
- `baseline '<g>' not found` — run `ghidra analyze` first (exit 2).
- `selector ... is ambiguous` — use a unique address/id (exit 2).
- `Ghidra is required ...` — external-required; run `ghidra doctor` (exit 2).
- `could not acquire lock ...` — another command holds the target lock (exit 32).
