# Script authoring

`ghidra script {scaffold,lint,run}` manages reusable headless Ghidra scripts.

## Location allowlist

Scripts live ONLY under:

- `<skill>/scripts/ghidra/` — bundled, reusable across targets.
- `<workspace>/artifacts/<id>/scripts/` — target-local.

Any path outside these roots is rejected. Symlinks (on the path or any parent)
and path traversal are rejected. Script names passed to `analyzeHeadless` must be
bare filenames resolving inside an allowlisted root.

## Language

- **Java is the guaranteed default.** Scaffolds extend `GhidraScript`.
- **Python requires PyGhidra.** `scaffold --language python` and running a `.py`
  script both require `doctor`-verified PyGhidra (launcher + importable library);
  otherwise `external-required` (exit 2) with install instructions.

## scaffold

```sh
ghidra script scaffold MyScan                      # Java, into <skill>/scripts/ghidra/
ghidra script scaffold MyScan --target demo        # into artifacts/demo/scripts/
ghidra script scaffold MyPy --language python       # needs PyGhidra
```

Scaffolds include the required `@category` metadata tag and a body that writes
only to a provided output directory.

## lint

```sh
ghidra script lint <path> [--target ID]
```

Checks: path inside the allowlist, no symlink/traversal, required Java metadata
(`@category`), `extends GhidraScript` for headless Java, and `@runtime PyGhidra`
for Python. Returns `{ok, findings}`.

## run

```sh
ghidra script run <path> --target ID [--arg V]... [--language java|python]
```

Runs lint first (a failing lint blocks the run), then executes the script via
`analyzeHeadless -process` against the target's project with the script's dir as
the only extra allowlisted root. Output and logs land under
`artifacts/<id>/scripts/runs/<run-id>.{json,log}`. `external-required` when
Ghidra (or PyGhidra for python) is missing.

## Bundled scripts

`ExportBaseline.java`, `DecompileFunction.java`, `ApplyMetadata.java`,
`VerifyMetadata.java`, and `ScriptProbe.java` are clean-room and use the Gson
bundled with Ghidra. They accept explicit arguments, write only into the given
output directory, and never execute the target.
