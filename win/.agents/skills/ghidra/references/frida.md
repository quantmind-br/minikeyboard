# Frida (optional, opt-in dynamic)

Frida is never required for static analysis and never blocks it. All Frida
operations are CLI/headless and manifest-driven. Traces are never fabricated.

## Commands

- `ghidra frida doctor` — report availability + isolation; `external-required`
  guidance when absent.
- `ghidra frida capture --target ID --scenario S [--trusted] [--isolation-profile FILE]`
- `ghidra frida trace ...` — same contract as capture.
- `ghidra frida import-evidence --target ID --manifest FILE` — import-only
  normalization of externally captured evidence.
- `ghidra frida compare --target ID [--static-ref R] [--runtime-ref R]` —
  static-vs-dynamic comparison; conflicts preserved until explicit adjudication.

## Scenarios

`signature | io | call-tree | dispatch-vtable | hotpath-coverage` — the five
first-class runtime evidence scenarios.

## Authorization & isolation

- Capture/trace require explicit authorization to run the target.
- **Trusted/owned targets** (`--trusted`): a fixture or target you authored may
  run with timeout + manifest + teardown.
- **Untrusted targets**: require an attested isolation profile. Without it,
  `capture`/`trace` refuse (exit 2) and you stay static. There is no unisolated
  local downgrade.

### Isolation profile

`--isolation-profile FILE` points at a **generated, verified** profile artifact
(not self-asserted booleans):

```json
{
  "schema_version": 1,
  "kind": "isolation-profile",
  "runtime": "docker",
  "config_path": "/abs/run-config",
  "config_sha256": "<64 hex>",
  "verifier": { "tool": "...", "evidence": "..." },
  "controls": {
    "network_none": true, "read_only_target": true, "dedicated_output": true,
    "non_root_user": true, "cap_drop_all": true, "seccomp": true,
    "cpu_limit": true, "memory_limit": true, "pids_limit": true,
    "disk_quota": true, "hard_timeout": true, "clean_env": true,
    "manifest": true, "teardown": true
  }
}
```

`doctor` validates this structure (schema, runtime, config hash, verifier,
controls) without executing anything; capture re-verifies the actual argv and
manifest against `config_sha256` before an untrusted run. The dedicated writable
output is `<workspace>/artifacts/<id>/runtime/`; everything else is read-only.

## Absence

Without Frida, every capture/trace returns `external-required` (exit 2) with:
`python -m pip install frida-tools`, venv guidance for externally-managed
Python, the official link, and the Linux `ptrace_scope` note. See
`security.md` for the full threat model and control list.
