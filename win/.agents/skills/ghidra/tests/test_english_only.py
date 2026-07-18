"""Enforce English-only user-facing text across the skill.

Scans Markdown, Python help/errors, Java, templates, and eval text for
unambiguous Portuguese markers. Technical names/commands are allowed; the check
targets prose words that would only appear in Portuguese.
"""
from __future__ import annotations

import re
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parent.parent

# Unambiguous Portuguese tokens (accented or function words) unlikely in English
# technical prose. Word-boundary matched, case-insensitive.
PT_MARKERS = [
    r"\bnão\b", r"\bção\b", r"\bçã", r"\barquivo\b", r"\banálise\b", r"\bfunção\b",
    r"\busuário\b", r"\bexecução\b", r"\bcomando\b", r"\bendereço\b", r"\bcabeçalho\b",
    r"\bbiblioteca\b", r"\bferramenta\b", r"\bsaída\b", r"\bentrada\b", r"\bcaminho\b",
    r"\bversão\b", r"\bcatálogo\b", r"\bproveniência\b", r"\bportuguês\b", r"\bdiretório\b",
    r"\bções\b", r"\bé\b", r"\bsão\b", r"\bnenhum\b", r"\bapenas\b", r"\btambém\b",
]
PT_RE = re.compile("|".join(PT_MARKERS), re.IGNORECASE)

SCAN_GLOBS = [
    "SKILL.md", "README.md", "capabilities.json",
    "references/*.md", "src/ghidra_skill/*.py", "scripts/*.py",
    "scripts/ghidra/*.java", "tests/*.py", "evals/*.py", "evals/*.json",
]

# The English-only test file itself contains PT markers as data; exclude it.
EXCLUDE = {"tests/test_english_only.py"}


def _files():
    seen = []
    for g in SCAN_GLOBS:
        for p in ROOT.glob(g):
            rel = str(p.relative_to(ROOT))
            if rel in EXCLUDE:
                continue
            seen.append(p)
    return seen


def test_no_portuguese_in_user_facing_text():
    offenders = {}
    for p in _files():
        text = p.read_text(encoding="utf-8", errors="replace")
        for i, line in enumerate(text.splitlines(), 1):
            m = PT_RE.search(line)
            if m:
                offenders.setdefault(str(p.relative_to(ROOT)), []).append((i, m.group(0), line.strip()[:80]))
    assert not offenders, f"Portuguese markers found: {offenders}"


def test_scan_covers_key_files():
    files = {str(p.relative_to(ROOT)) for p in _files()}
    assert "SKILL.md" in files
    assert any(f.startswith("references/") for f in files)
    assert any(f.startswith("src/ghidra_skill/") for f in files)
