"""In-process CLI invocation helper for tests."""
from __future__ import annotations

import io
import json
from contextlib import redirect_stdout, redirect_stderr

from ghidra_skill.cli import main


def run_cli(*argv: str) -> tuple[int, dict, str]:
    """Run the CLI in-process. Returns (exit_code, parsed_json_stdout, stderr)."""
    out, err = io.StringIO(), io.StringIO()
    with redirect_stdout(out), redirect_stderr(err):
        try:
            code = main(list(argv))
        except SystemExit as e:  # argparse usage errors
            code = int(e.code) if e.code is not None else 0
    text = out.getvalue()
    try:
        data = json.loads(text) if text.strip() else {}
    except json.JSONDecodeError:
        data = {"_raw": text}
    return code, data, err.getvalue()
