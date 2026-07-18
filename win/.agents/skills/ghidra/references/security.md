# Security: threat model & native isolation controls

This is a **native** security policy specific to Ghidra/Frida reverse
engineering. It is **not** the generic `shared/sandbox-contract.md` and must not
be reused from it; native binaries and dynamic instrumentation demand controls
that a text-processing sandbox does not.

## Threat model

- **Untrusted sample = hostile code.** A binary under analysis may attempt to
  execute payloads, phone home, escape a weak sandbox, corrupt the workspace, or
  exhaust resources. Treat every non-owned target as hostile until proven
  otherwise.
- **Static analysis surface.** Import/auto-analysis/decompilation parse
  attacker-controlled bytes with Ghidra. Ghidra does **not execute** the target,
  but malformed inputs are still parsed — keep artifact writes confined and
  never run the target as a side effect.
- **Dynamic analysis surface.** Frida attach/spawn, project rebuild, and any
  compiled harness **execute code**. This is where real compromise happens.

## Operation classes

Every operation is classified into one of four tiers. `doctor` reports which
controls are observable and sets `dynamic_ready`.

| Tier | Operations | Executes target? | Controls |
|---|---|---|---|
| `static-read` | `inspect`, hashing, `list`, `show`, `report`, `validate` | no | native read-only tools; no target execution |
| `static-ghidra` | `analyze`, `decompile`, `metadata apply`, `function analyze`, `script run` | no | Ghidra parses bytes; writes confined to the target artifact dir; allowlisted script output |
| `dynamic-owned` | `frida capture`/`trace` on a fixture or target you own and authored | yes (owned) | authorization + timeout + artifact manifest; teardown |
| `dynamic-untrusted` | `frida capture`/`trace`/rebuild/harness on any non-owned or suspect sample | yes (hostile) | full isolation (below); if unattestable, **refuse** and stay static |

Frida attach/spawn, project rebuild, and harness compilation/execution inherit
`dynamic-untrusted` whenever the sample is not owned.

## Required controls for `dynamic-untrusted`

All of the following MUST be attested before any untrusted dynamic run. If any
cannot be verified, `doctor` returns `dynamic_ready: false` and the operation
refuses to run.

- **Ephemeral isolation:** a throwaway VM or container destroyed after the run.
- **Network blocked:** no egress (no default route / `--network none`).
- **Read-only target & inputs:** the sample and any input mount read-only.
- **Dedicated output dir:** the only writable mount is the target's runtime
  artifact directory `<workspace>/artifacts/<id>/runtime/` (bind-mounted
  writable into the sandbox); all other paths, including the rest of the
  workspace, stay read-only.
- **Non-root user:** run as an unprivileged uid; no privilege escalation.
- **Empty capabilities:** all Linux capabilities dropped
  (`--cap-drop ALL` / equivalent).
- **seccomp:** a restrictive seccomp profile applied.
- **Resource quotas:** bounded CPU, RAM, PIDs, and disk.
- **Hard timeout:** wall-clock kill; no unbounded runs.
- **Clean environment:** no host secrets/env leaked into the sandbox.
- **Manifest:** record image/rootfs, mounts, limits, argv, timestamps, hashes.
- **Teardown:** destroy the sandbox and confirm removal.

## Enforcement in the CLI

- `doctor` probes the observable controls (container/VM runtime availability,
  ability to drop capabilities, seccomp support, network-none, resource limits)
  and sets `dynamic_ready` accordingly with per-control detail.
- `frida capture`/`trace` refuse with exit `2` when the target is untrusted and
  `dynamic_ready` is false, returning the missing controls.
- Static tiers never depend on these controls and remain available when Frida or
  isolation is absent.
- There is **no** downgrade path that runs an untrusted sample locally without
  isolation. Suspect targets stay static.
