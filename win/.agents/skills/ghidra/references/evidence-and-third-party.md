# Evidence and third-party identification

## Principle

Always separate **observed** (directly seen in an artifact), **inferred**
(reasoned from evidence), and **unresolved** (open questions). Reports and
compares preserve this separation and never collapse a conflict into a single
claim.

## Third-party libraries

```sh
ghidra evidence third-party --target ID --library zlib --version 1.3.1 \
  --source /path/to/pristine --confidence high \
  --evidence "string 'inflate 1.3.1'" --classification observed
ghidra evidence third-party --target ID --none      # reviewed, none found
ghidra evidence third-party --target ID --list
```

- `--confidence low|medium|high`, `--classification observed|inferred|unresolved`.
- Pristine sources are recorded by **path + hash only**. Files are hashed
  (single file: `sha256`; directory: a tree hash over sorted per-file digests)
  and never modified or fetched. Acquisition is out of scope for the CLI.
- Records land in `evidence/third-party.json`.

## Static vs. dynamic conflict

When a static baseline and imported Frida runtime evidence disagree, `ghidra
frida compare` keeps both surfaces under `conflicts` until a reviewer records an
explicit adjudication. Neither side is silently dropped.

## Progressive decompilation compare

`ghidra compare` requires all of `--reason`, `--question`, `--boundary`,
`--fallback`, `--compare`. This forces each outside-in decompilation step to
state why it runs, what it answers, the replaced boundary, the fallback if it
fails, and what it compares against — so a step is auditable as `ready`,
`qualified`, or `blocked`.

## Improvement review

`ghidra improve review` records a reusable-improvement candidate as
`accepted | deferred | rejected` with evidence, overlap, and a proposed
destination. It never self-edits the skill; promotion to skill files is a
separate, reviewed maintenance change.
