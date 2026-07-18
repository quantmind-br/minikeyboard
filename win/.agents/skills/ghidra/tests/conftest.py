"""Shared test fixtures.

Adds src/ to sys.path, provides a temp workspace, a built ELF fixture, and a
FAKE analyzeHeadless that only validates argv/allowlist/output and writes
placeholder baseline JSON. The fake NEVER counts as proof of real analysis; it
exists to exercise CLI orchestration deterministically without Ghidra.
"""
from __future__ import annotations

import json
import os
import subprocess
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parent.parent
SRC = ROOT / "src"
sys.path.insert(0, str(SRC))


@pytest.fixture
def workspace(tmp_path):
    return tmp_path / "ws"


@pytest.fixture
def elf_c(tmp_path):
    """Compile the ELF C fixture into tmp; skip if no compiler."""
    import shutil
    gcc = shutil.which("gcc")
    if not gcc:
        pytest.skip("gcc not available")
    src = ROOT / "tests" / "fixtures" / "src" / "elf_c" / "prog.c"
    out = tmp_path / "elf_c"
    subprocess.run([gcc, "-O1", "-fno-pic", "-no-pie", "-o", str(out), str(src)],
                   check=True, env=dict(os.environ, LC_ALL="C"))
    return out


@pytest.fixture
def fake_ghidra(tmp_path, monkeypatch):
    """Install a fake analyzeHeadless on a discovered Ghidra home.

    Returns a callable installing the fake and a log path capturing argv.
    """
    home = tmp_path / "ghidra-home"
    (home / "support").mkdir(parents=True)
    (home / "Ghidra").mkdir(parents=True)
    (home / "Ghidra" / "application.properties").write_text(
        "application.version=12.1.2\napplication.release.name=DEV\n")
    argv_log = tmp_path / "fake-argv.json"

    fake = home / "support" / "analyzeHeadless"
    baseline_names = ["functions", "callgraph", "types", "vtables",
                      "constants", "strings", "imports"]
    fake.write_text(f"""#!/usr/bin/env python3
import sys, json, os
from pathlib import Path
argv = sys.argv[1:]
json.dump(argv, open({str(argv_log)!r}, "w"))
# find -postScript ExportBaseline.java <outdir> and emit placeholder baselines
for i, a in enumerate(argv):
    if a == "-postScript" and i + 1 < len(argv) and argv[i+1] == "ExportBaseline.java":
        outdir = Path(argv[i+2])
        outdir.mkdir(parents=True, exist_ok=True)
        for n in {baseline_names!r}:
            json.dump({{"schema_version":1,"program":"fake","count":0,n:[]}},
                      open(outdir / (n+".json"), "w"))
    if a == "-postScript" and i + 1 < len(argv) and argv[i+1] == "DecompileFunction.java":
        outdir = Path(argv[i+3])
        outdir.mkdir(parents=True, exist_ok=True)
        (outdir / "source.c").write_text("int fake(){{return 0;}}\\n")
        json.dump({{"decompiled":True}}, open(outdir / "analysis.json","w"))
sys.exit(0)
""")
    fake.chmod(0o755)
    monkeypatch.setenv("GHIDRA_HOME", str(home))
    return {"home": home, "argv_log": argv_log, "analyze_headless": fake}
