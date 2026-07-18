"""Structured stdout envelope + exit codes + typed errors.

Envelope shape (stdout, always JSON when --format json):
    { "status": "ok|error", "message": str, "data": {...}, "artifacts": [...] }

Logs go to stderr, never stdout. Exit codes:
    0  success
    1  operational failure
    2  usage / validation error AND external-required (dependency missing)
    32 lock acquisition timeout

`status` is strictly "ok" or "error". A missing external dependency does NOT
change the status enum; it is reported as an error with
`data.reason == "external-required"` plus `data.dependency` and
`data.diagnostic`, and maps to exit code 2.
"""
from __future__ import annotations

import json
import sys
from dataclasses import dataclass, field
from typing import Any

EXIT_OK = 0
EXIT_FAIL = 1
EXIT_USAGE = 2
EXIT_LOCK_TIMEOUT = 32

REASON_EXTERNAL_REQUIRED = "external-required"


@dataclass
class Envelope:
    status: str  # "ok" | "error"
    message: str
    data: dict[str, Any] = field(default_factory=dict)
    artifacts: list[str] = field(default_factory=list)
    exit_code: int = EXIT_OK

    def to_dict(self) -> dict[str, Any]:
        return {
            "status": self.status,
            "message": self.message,
            "data": self.data,
            "artifacts": self.artifacts,
        }


def ok(message: str, data: dict[str, Any] | None = None,
       artifacts: list[str] | None = None) -> Envelope:
    return Envelope("ok", message, data or {}, artifacts or [], EXIT_OK)


def error(message: str, data: dict[str, Any] | None = None,
          exit_code: int = EXIT_FAIL,
          artifacts: list[str] | None = None) -> Envelope:
    return Envelope("error", message, data or {}, artifacts or [], exit_code)


def external_required(message: str, dependency: str, diagnostic: str,
                      extra: dict[str, Any] | None = None) -> Envelope:
    data: dict[str, Any] = {
        "reason": REASON_EXTERNAL_REQUIRED,
        "dependency": dependency,
        "diagnostic": diagnostic,
    }
    if extra:
        data.update(extra)
    return Envelope("error", message, data, [], EXIT_USAGE)


class SkillError(Exception):
    """Base for CLI errors carrying an exit code + envelope data."""

    exit_code = EXIT_FAIL
    reason: str | None = None

    def __init__(self, message: str, data: dict[str, Any] | None = None):
        super().__init__(message)
        self.message = message
        self.data = data or {}

    def to_envelope(self) -> Envelope:
        data = dict(self.data)
        if self.reason and "reason" not in data:
            data["reason"] = self.reason
        return error(self.message, data, self.exit_code)


class UsageError(SkillError):
    exit_code = EXIT_USAGE
    reason = "usage"


class ValidationError(SkillError):
    exit_code = EXIT_USAGE
    reason = "validation"


class LockTimeout(SkillError):
    exit_code = EXIT_LOCK_TIMEOUT
    reason = "lock-timeout"


class ExternalRequired(SkillError):
    exit_code = EXIT_USAGE
    reason = REASON_EXTERNAL_REQUIRED

    def __init__(self, message: str, dependency: str, diagnostic: str,
                 extra: dict[str, Any] | None = None):
        data = {"dependency": dependency, "diagnostic": diagnostic}
        if extra:
            data.update(extra)
        super().__init__(message, data)


def _check_invariants(env: Envelope) -> None:
    if env.status not in ("ok", "error"):
        raise ValueError(f"envelope.status must be ok|error, got {env.status!r}")
    if env.status == "ok" and env.exit_code != EXIT_OK:
        raise ValueError(f"ok envelope must use exit code {EXIT_OK}, got {env.exit_code}")
    if env.status == "error" and env.exit_code == EXIT_OK:
        raise ValueError("error envelope must use a non-zero exit code")
    if not isinstance(env.data, dict):
        raise ValueError("envelope.data must be a dict")
    if not isinstance(env.artifacts, list):
        raise ValueError("envelope.artifacts must be a list")


def emit(env: Envelope, fmt: str = "json", stream=None) -> int:
    """Write the envelope to stdout in the requested format; return exit code."""
    _check_invariants(env)
    if fmt not in ("json", "text"):
        raise ValueError(f"format must be json|text, got {fmt!r}")
    out = stream or sys.stdout
    if fmt == "text":
        _emit_text(env, out)
    else:
        json.dump(env.to_dict(), out, indent=2, sort_keys=False)
        out.write("\n")
    return env.exit_code


def _emit_text(env: Envelope, out) -> None:
    out.write(f"[{env.status}] {env.message}\n")
    if env.data:
        for k, v in env.data.items():
            out.write(f"  {k}: {_short(v)}\n")
    for a in env.artifacts:
        out.write(f"  artifact: {a}\n")


def _short(v: Any) -> str:
    s = json.dumps(v) if not isinstance(v, str) else v
    return s if len(s) <= 200 else s[:197] + "..."
