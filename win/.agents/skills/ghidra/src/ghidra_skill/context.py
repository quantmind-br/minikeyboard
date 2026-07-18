"""Runtime context shared by all command handlers."""
from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path

from .workspace import Workspace


@dataclass
class Context:
    workspace: Workspace
    fmt: str = "json"
    target: str | None = None
    timeout: float = 600.0
    lock_timeout: float = 30.0
    no_wait: bool = False
    ghidra_home: str | None = None

    @property
    def ws(self) -> Workspace:
        return self.workspace

    def resolve_target(self, explicit: str | None = None) -> str:
        tid = explicit or self.target
        if not tid:
            from .envelope import UsageError
            raise UsageError("--target is required for this command")
        return tid
