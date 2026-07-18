#!/usr/bin/env python3
"""Deterministic clean-room fixture builder.

Builds the test fixtures into an output directory (never the source tree) and
writes a manifest with compiler, version, flags, and SHA-256 for each artifact.
The C++ vtable fixture is pinned to clang++ (with a normal and a stripped
variant) so the vtable-recovery test can assert invariants deterministically.

Usage: python scripts/build_fixtures.py --out DIR
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SRC = ROOT / "tests" / "fixtures" / "src"

# Deterministic flags. -O1 keeps noinline functions distinct; -fno-PIC/-no-pie
# for stable ELF layout; frame pointers for cleaner decompilation.
C_FLAGS = ["-O1", "-fno-pic", "-no-pie", "-fno-omit-frame-pointer", "-g0"]
CPP_FLAGS = ["-O1", "-fno-pic", "-no-pie", "-fno-omit-frame-pointer", "-g0"]


def _sha256(p: Path) -> str:
    return hashlib.sha256(p.read_bytes()).hexdigest()


def _version(tool: str) -> str:
    env = dict(os.environ, LC_ALL="C")
    try:
        out = subprocess.run([tool, "--version"], capture_output=True, text=True,
                             env=env, timeout=10).stdout
        return out.splitlines()[0].strip() if out.strip() else "unknown"
    except (OSError, subprocess.SubprocessError):
        return "missing"


def _run(cmd: list[str]) -> None:
    env = dict(os.environ, LC_ALL="C")
    subprocess.run(cmd, check=True, env=env)


def _require(tool: str) -> str:
    path = shutil.which(tool)
    if not path:
        raise SystemExit(f"required tool not found: {tool}")
    return path


def build(out: Path) -> dict:
    out.mkdir(parents=True, exist_ok=True)
    gcc = _require("gcc")
    clang = _require("clang")
    clangpp = _require("clang++")
    ar = _require("ar")

    artifacts: dict[str, dict] = {}

    # 1. ELF C: main -> bridge -> hot
    elf_c = out / "elf_c"
    _run([gcc, *C_FLAGS, "-o", str(elf_c), str(SRC / "elf_c" / "prog.c")])
    artifacts["elf_c"] = _entry(elf_c, "gcc", _version("gcc"), C_FLAGS)

    # 2. ELF C++ (clang++) normal + stripped
    elf_cpp = out / "elf_cpp"
    _run([clangpp, *CPP_FLAGS, "-o", str(elf_cpp), str(SRC / "elf_cpp" / "shapes.cpp")])
    artifacts["elf_cpp"] = _entry(elf_cpp, "clang++", _version("clang++"), CPP_FLAGS)
    elf_cpp_stripped = out / "elf_cpp_stripped"
    shutil.copyfile(elf_cpp, elf_cpp_stripped)
    _run([_require("strip"), str(elf_cpp_stripped)])
    artifacts["elf_cpp_stripped"] = _entry(
        elf_cpp_stripped, "clang++ + strip", _version("clang++"), CPP_FLAGS + ["strip"])

    # 3. shared library + consumer
    libdir = out
    libmath = libdir / "libmath.so"
    _run([clang, "-O1", "-fpic", "-shared", "-o", str(libmath),
          str(SRC / "shared" / "mathlib.c")])
    artifacts["libmath.so"] = _entry(libmath, "clang", _version("clang"),
                                     ["-O1", "-fpic", "-shared"])
    consumer = out / "consumer"
    _run([clang, "-O1", "-fno-pic", "-no-pie", "-o", str(consumer),
          str(SRC / "shared" / "consumer.c"), f"-I{SRC/'shared'}",
          f"-L{libdir}", "-lmath", f"-Wl,-rpath,{libdir}"])
    artifacts["consumer"] = _entry(consumer, "clang", _version("clang"),
                                   ["-O1", "-fno-pic", "-no-pie", "-lmath"])

    # 4. multi-member archive with an invalid member appended
    obj_a = out / "member_a.o"
    obj_b = out / "member_b.o"
    _run([gcc, "-O1", "-c", "-o", str(obj_a), str(SRC / "archive" / "member_a.c")])
    _run([gcc, "-O1", "-c", "-o", str(obj_b), str(SRC / "archive" / "member_b.c")])
    lib_a = out / "libpair.a"
    if lib_a.exists():
        lib_a.unlink()
    _run([ar, "rcs", str(lib_a), str(obj_a), str(obj_b)])
    # append a bogus "object" member with a .o name so `ar t` lists it but nm fails
    bogus = out / "broken.o"
    bogus.write_bytes(b"not a real object file\n")
    _run([ar, "q", str(lib_a), str(bogus)])
    artifacts["libpair.a"] = _entry(lib_a, "ar", _version("ar"),
                                    ["rcs member_a.o member_b.o", "q broken.o"])

    # 5. invalid / truncated binary
    bad = out / "truncated.bin"
    bad.write_bytes(b"\x7fELF" + b"\x00" * 12)  # ELF magic then garbage/truncated
    artifacts["truncated.bin"] = _entry(bad, "handwritten", "n/a", ["truncated-elf"])

    manifest = {
        "schema_version": 1,
        "toolchain": {
            "gcc": _version("gcc"), "clang": _version("clang"),
            "clang++": _version("clang++"), "ar": _version("ar"),
        },
        "artifacts": artifacts,
    }
    (out / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    return manifest


def _entry(path: Path, compiler: str, version: str, flags: list[str]) -> dict:
    return {
        "path": str(path),
        "compiler": compiler,
        "compiler_version": version,
        "flags": flags,
        "sha256": _sha256(path),
        "size": path.stat().st_size,
    }


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", required=True, help="output directory (not the source tree)")
    args = ap.parse_args()
    out = Path(args.out).resolve()
    if SRC in out.parents or out == SRC:
        raise SystemExit("refusing to build into the fixture source tree")
    manifest = build(out)
    print(json.dumps(manifest, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
