# Workflow: intake through decompile iterations

The `ghidra` skill drives one flow:

```
doctor -> init -> analyze -> inspect / list / show -> metadata -> decompile -> compare / report -> validate
```

## 1. doctor

Always first. `ghidra doctor --format json` reports Ghidra/JDK/Python/binutils/
Frida/isolation and, per missing dependency, an install route. It never mutates
the system. `dynamic_ready` is false unless a verified isolation profile is
supplied (see `frida.md`).

## 2. init

```sh
ghidra init ./target --target demo --scope full
```

Validates the file, computes SHA-256, inspects the format (no execution), and
creates `targets/demo/projects/` plus `artifacts/demo/`. Scope modes:
`full | symbols | addresses`; refine later with `ghidra config scope`.

## 3. analyze

```sh
ghidra analyze --target demo [--rebuild] [--processor X] [--cspec Y] [--analysis-timeout N]
```

Imports the binary into Ghidra and auto-analyzes via `analyzeHeadless`, then runs
`ExportBaseline.java` to write the seven baselines. State -> `analyzed`.
`external-required` when Ghidra is absent.

## 4. inspect / list / show

Read structured baselines without re-running Ghidra:

```sh
ghidra list functions --target demo
ghidra list callgraph --target demo --callers --selector 0x00101160 --transitive
ghidra show function --target demo main
```

Callgraph is direct-call edges only (see `cli.md`).

## 5. metadata

```sh
ghidra metadata rename --target demo --address 0x00101160 --new-name parse_header --provenance "string xref"
ghidra metadata apply --target demo
```

`rename|signature|types` record intent with provenance. `apply` mutates the
project via `ApplyMetadata.java` and re-exports with `VerifyMetadata.java`.
Conflicts require `--force`. State -> `enriched`.

## 6. decompile

```sh
ghidra decompile --target demo --function main
ghidra decompile --target demo --batch batch.json     # {"functions": ["main","0x00101160"]}
ghidra function analyze --target demo main             # strict 5-step order
```

Selectors resolve uniquely first; ambiguous selectors fail before running. Batch
preserves partial successes. State -> `decompiled`.

## 7. compare / report / validate

```sh
ghidra compare --target demo --reason R --question Q --boundary B --fallback F --compare C
ghidra report --target demo
ghidra validate --target demo
```

`validate` computes gates from artifacts and sets state -> `validated` when all
required gates pass. Gates not exercised are `not_applicable`.

## State machine

```
initialized -> analyzed -> enriched -> decompiled -> validated
                                                   \-> failed
```

Legacy imported P0-P6 states are translated in `validate` output, never
reintroduced as public commands.
