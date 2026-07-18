"""Script scaffold/lint safety tests (no real Ghidra)."""
from __future__ import annotations

import os
from pathlib import Path

import pytest

from ghidra_skill import script_ops
from ghidra_skill.context import Context
from ghidra_skill.envelope import ExternalRequired, ValidationError, UsageError
from ghidra_skill.workspace import Workspace


def _ctx(tmp_path, elf_c):
    ws = Workspace(tmp_path / "ws")
    ws.create_target("demo", elf_c, "full", [])
    return Context(workspace=ws)


def test_scaffold_java_default(tmp_path, elf_c):
    ctx = _ctx(tmp_path, elf_c)
    res = script_ops.scaffold(ctx, "MyScan", target="demo")
    p = Path(res["path"])
    assert res["language"] == "java" and p.suffix == ".java"
    text = p.read_text()
    assert "@category" in text and "extends GhidraScript" in text
    # confined to the target scripts dir
    assert "artifacts/demo/scripts" in str(p)


def test_lint_flags_missing_metadata(tmp_path, elf_c):
    ctx = _ctx(tmp_path, elf_c)
    scripts = ctx.ws.sub("demo", "scripts")
    scripts.mkdir(parents=True, exist_ok=True)
    bad = scripts / "Bad.java"
    bad.write_text("public class Bad {}\n")  # no @category, no GhidraScript
    res = script_ops.lint(ctx, str(bad), target="demo")
    assert res["ok"] is False
    assert any("@category" in f for f in res["findings"])


def test_lint_rejects_traversal(tmp_path, elf_c):
    ctx = _ctx(tmp_path, elf_c)
    outside = tmp_path / "evil.java"
    outside.write_text("// @category X\npublic class evil extends GhidraScript {}\n")
    with pytest.raises(ValidationError):
        script_ops.lint(ctx, str(outside), target="demo")


def test_lint_rejects_symlink(tmp_path, elf_c):
    ctx = _ctx(tmp_path, elf_c)
    scripts = ctx.ws.sub("demo", "scripts")
    scripts.mkdir(parents=True, exist_ok=True)
    real = tmp_path / "real.java"
    real.write_text("// @category X\npublic class real extends GhidraScript {}\n")
    link = scripts / "link.java"
    try:
        link.symlink_to(real)
    except OSError:
        pytest.skip("symlinks unsupported")
    with pytest.raises(ValidationError):
        script_ops.lint(ctx, str(link), target="demo")


def test_python_run_requires_pyghidra(tmp_path, elf_c, monkeypatch):
    ctx = _ctx(tmp_path, elf_c)
    from ghidra_skill import script_ops as so
    monkeypatch.setattr(so, "check_pyghidra",
                        lambda gh: {"launcher": True, "library": False, "ready": False})
    monkeypatch.setattr(so, "discover_ghidra",
                        lambda home=None: {"found": True, "home": "/opt/ghidra",
                                           "pyghidra_run": True})
    scripts = ctx.ws.sub("demo", "scripts")
    scripts.mkdir(parents=True, exist_ok=True)
    py = scripts / "s.py"
    py.write_text("# @runtime PyGhidra\nprint('x')\n")
    with pytest.raises(ExternalRequired):
        so.run(ctx, str(py), target="demo", language="python")
