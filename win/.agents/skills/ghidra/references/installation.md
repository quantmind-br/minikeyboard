# Installation matrix

`ghidra doctor` reports exactly what is missing with official URLs and per-OS
commands. Package-manager commands are convenience only; the skill never runs
them and always offers a manual route. Only detected managers are suggested.

## Ghidra + JDK 21

- Official: <https://github.com/NationalSecurityAgency/ghidra/releases>
- JDK 21 (64-bit): <https://adoptium.net/temurin/releases/?version=21>
- Manual: download `ghidra_<version>_PUBLIC` ZIP, extract, install a 64-bit
  JDK 21, set `GHIDRA_HOME` to the extracted dir (must contain
  `support/analyzeHeadless`).
- Baseline verified for this skill: Ghidra 12.x, JDK 21+.
- macOS: `brew install --cask ghidra`, `brew install openjdk@21`.
- Windows: `winget install --id GhidraSRE.Ghidra`,
  `winget install --id EclipseAdoptium.Temurin.21.JDK`.
- Linux JDK: `apt-get install openjdk-21-jdk` / `dnf install java-21-openjdk-devel`
  / `pacman -S jdk21-openjdk`. Ghidra itself is downloaded from the release page.

## Python 3.12+ / venv / pip

- Official: <https://www.python.org/downloads/>
- Linux: `apt-get install python3 python3-venv python3-pip` /
  `dnf install python3 python3-pip` / `pacman -S python python-pip`.
- macOS: `brew install python@3.12`. Windows: `winget install Python.Python.3.12`.
- If Python is externally managed, create a venv:
  `python3 -m venv .venv && . .venv/bin/activate`.

## PyGhidra (optional, for python scripts)

- Docs/artifacts ship inside the Ghidra release under
  `Ghidra/Features/PyGhidra/`.
- Online: `python -m pip install pyghidra`
- Offline: `python -m pip install --no-index -f
  <Ghidra>/Ghidra/Features/PyGhidra/pypkg/dist pyghidra`
- Prefer a venv; set `GHIDRA_INSTALL_DIR` to the Ghidra home.
- `doctor` distinguishes the launcher (`support/pyghidraRun`) from the importable
  library; both are required for `ready`.

## binutils + file + compilers

- Linux: `apt-get install binutils file` / `dnf install binutils file` /
  `pacman -S binutils file`. Compilers: gcc/clang for fixtures.
- macOS: `brew install binutils`. Windows: `winget install GnuWin32.Binutils`.
- `inspect` degrades gracefully if these are missing; `analyze`/`decompile` still
  require Ghidra.

## Frida (optional, dynamic)

- Official: <https://frida.re/docs/installation/>
- `python -m pip install frida-tools` (venv recommended).
- On Linux, set `/proc/sys/kernel/yama/ptrace_scope` to `0` (or use sudo) to
  allow attach.

## Isolation (untrusted dynamic runs)

- A container runtime (docker/podman) or VM (qemu/firecracker).
- `dynamic_ready` requires a **verified isolation profile** (see `security.md`
  and `frida.md`); runtime presence alone is insufficient.
