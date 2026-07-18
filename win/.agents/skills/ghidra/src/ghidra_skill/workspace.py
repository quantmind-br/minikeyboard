"""Workspace, target resolution, per-target exclusive locking, and scope.

Layout (under <workspace>):
    targets/<id>/projects/                Ghidra project dir (created lazily)
    artifacts/<id>/
        state.json
        intake/          baseline/        evidence/
        metadata/        decompilation/   runtime/
        scripts/         reports/         gates/
        analysis/        locks/           execution-log.jsonl

Locks: a per-target lockfile opened O_EXCL with a bounded acquisition timeout.
"""
from __future__ import annotations

import errno
import hashlib
import os
import re
import time
from contextlib import contextmanager
from pathlib import Path
from typing import Any

from . import SCHEMA_VERSION
from .artifacts import now_iso, read_json, write_json
from .envelope import LockTimeout, UsageError, ValidationError

STATES = ("initialized", "analyzed", "enriched", "decompiled", "validated", "failed")
SCOPE_MODES = ("full", "symbols", "addresses")

# Deterministic ASCII target ids: 1..64 chars, alnum boundaries, . _ - inside.
TARGET_RE = re.compile(r"[A-Za-z0-9](?:[A-Za-z0-9._-]{0,62}[A-Za-z0-9])?", re.ASCII)
SHA256_RE = re.compile(r"[0-9a-f]{64}", re.ASCII)

ARTIFACT_SUBDIRS = (
    "intake", "baseline", "evidence", "metadata", "decompilation",
    "runtime", "scripts", "reports", "gates", "analysis", "locks",
)


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


class Workspace:
    def __init__(self, root: Path):
        self.root = Path(root).resolve()

    # ---- paths -----------------------------------------------------------
    def target_dir(self, target_id: str) -> Path:
        return self.root / "targets" / target_id

    def project_dir(self, target_id: str) -> Path:
        return self.target_dir(target_id) / "projects"

    def artifact_dir(self, target_id: str) -> Path:
        return self.root / "artifacts" / target_id

    def state_path(self, target_id: str) -> Path:
        return self.artifact_dir(target_id) / "state.json"

    def sub(self, target_id: str, name: str) -> Path:
        return self.artifact_dir(target_id) / name

    def execution_log(self, target_id: str) -> Path:
        return self.artifact_dir(target_id) / "execution-log.jsonl"

    # ---- validation ------------------------------------------------------
    @staticmethod
    def valid_target_id(target_id: str) -> bool:
        # Deterministic ASCII names: start/end alphanumeric, interior may add
        # . _ -, total length 1..64. Rejects "", ".", "..", traversal, path
        # separators, and Unicode confusables.
        return bool(TARGET_RE.fullmatch(target_id or ""))

    def require_target(self, target_id: str) -> Path:
        if not self.valid_target_id(target_id):
            raise UsageError(f"invalid target id: {target_id!r}")
        sp = self.state_path(target_id)
        if not sp.is_file():
            raise UsageError(f"target not found: {target_id!r} (run `ghidra init` first)")
        return sp

    # ---- create ----------------------------------------------------------
    def create_target(self, target_id: str, binary: Path, scope_mode: str,
                       entries: list[str]) -> dict[str, Any]:
        if not self.valid_target_id(target_id):
            raise UsageError(f"invalid target id: {target_id!r}")
        if scope_mode not in SCOPE_MODES:
            raise UsageError(f"invalid scope mode: {scope_mode!r} (use {'|'.join(SCOPE_MODES)})")
        binary = Path(binary)
        if not binary.is_file():
            raise UsageError(f"binary not found: {binary}")
        if self.state_path(target_id).exists():
            raise UsageError(f"target already exists: {target_id!r}")

        self.project_dir(target_id).mkdir(parents=True, exist_ok=True)
        adir = self.artifact_dir(target_id)
        adir.mkdir(parents=True, exist_ok=True)
        for name in ARTIFACT_SUBDIRS:
            (adir / name).mkdir(parents=True, exist_ok=True)

        digest = sha256_file(binary)
        state = {
            "schema_version": SCHEMA_VERSION,
            "target_id": target_id,
            "binary": {
                "path": str(binary.resolve()),
                "sha256": digest,
                "format": None,       # filled by inspect/init
            },
            "scope": {"mode": scope_mode, "entries": list(entries)},
            "status": "initialized",
            "created_at": now_iso(),
            "updated_at": now_iso(),
        }
        write_json(self.state_path(target_id), state)
        return state

    # ---- state -----------------------------------------------------------
    def _validate_state(self, state: Any, target_id: str) -> dict[str, Any]:
        if not isinstance(state, dict):
            raise ValidationError(f"corrupt state for target {target_id!r}: not an object")
        if state.get("target_id") != target_id:
            raise ValidationError(
                f"state integrity error: file for {target_id!r} declares "
                f"target_id={state.get('target_id')!r}")
        if not self.valid_target_id(state.get("target_id") or ""):
            raise ValidationError(f"invalid target id in state: {state.get('target_id')!r}")
        if state.get("schema_version") != SCHEMA_VERSION:
            raise ValidationError(
                f"unsupported state schema_version {state.get('schema_version')!r} "
                f"for target {target_id!r} (expected {SCHEMA_VERSION})")
        if state.get("status") not in STATES:
            raise ValidationError(
                f"invalid state status {state.get('status')!r} for target {target_id!r}")
        binary = state.get("binary")
        if not isinstance(binary, dict):
            raise ValidationError(f"corrupt state for target {target_id!r}: binary must be an object")
        bpath = binary.get("path")
        if not isinstance(bpath, str) or not bpath:
            raise ValidationError(f"corrupt state for target {target_id!r}: binary.path must be a non-empty string")
        if not SHA256_RE.fullmatch(binary.get("sha256") or ""):
            raise ValidationError(f"corrupt state for target {target_id!r}: binary.sha256 must be 64 lowercase hex chars")
        if "format" not in binary:
            raise ValidationError(f"corrupt state for target {target_id!r}: binary.format key required (nullable)")
        scope = state.get("scope")
        if not isinstance(scope, dict) or scope.get("mode") not in SCOPE_MODES:
            raise ValidationError(
                f"corrupt state for target {target_id!r}: invalid scope.mode")
        entries = scope.get("entries")
        if not isinstance(entries, list) or not all(isinstance(e, str) for e in entries):
            raise ValidationError(
                f"corrupt state for target {target_id!r}: scope.entries must be a list of strings")
        return state

    def load_state(self, target_id: str) -> dict[str, Any]:
        return self._validate_state(read_json(self.require_target(target_id)), target_id)

    def save_state(self, state: dict[str, Any]) -> None:
        tid = state.get("target_id")
        if not self.valid_target_id(tid or ""):
            raise ValidationError(f"invalid target id in state: {tid!r}")
        self._validate_state(state, tid)
        state["updated_at"] = now_iso()
        write_json(self.state_path(tid), state)

    def set_status(self, target_id: str, status: str) -> dict[str, Any]:
        if status not in STATES:
            raise ValidationError(f"invalid state: {status!r}")
        state = self.load_state(target_id)
        state["status"] = status
        self.save_state(state)
        return state

    # ---- scope -----------------------------------------------------------
    def scope_show(self, target_id: str) -> dict[str, Any]:
        return self.load_state(target_id)["scope"]

    def scope_set(self, target_id: str, mode: str, entries: list[str]) -> dict[str, Any]:
        if mode not in SCOPE_MODES:
            raise UsageError(f"invalid scope mode: {mode!r}")
        state = self.load_state(target_id)
        state["scope"] = {"mode": mode, "entries": list(entries)}
        self.save_state(state)
        return state["scope"]

    def scope_add(self, target_id: str, entry: str) -> dict[str, Any]:
        state = self.load_state(target_id)
        if entry not in state["scope"]["entries"]:
            state["scope"]["entries"].append(entry)
        self.save_state(state)
        return state["scope"]

    def scope_remove(self, target_id: str, entry: str) -> dict[str, Any]:
        state = self.load_state(target_id)
        state["scope"]["entries"] = [e for e in state["scope"]["entries"] if e != entry]
        self.save_state(state)
        return state["scope"]

    # ---- lock ------------------------------------------------------------
    @contextmanager
    def lock(self, target_id: str, timeout: float = 30.0, no_wait: bool = False):
        lock_path = self.sub(target_id, "locks") / f"{target_id}.lock"
        lock_path.parent.mkdir(parents=True, exist_ok=True)
        deadline = time.monotonic() + max(0.0, timeout)
        fd = None
        while True:
            try:
                fd = os.open(str(lock_path), os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o644)
                os.write(fd, f"{os.getpid()} {now_iso()}\n".encode())
                break
            except OSError as e:
                if e.errno != errno.EEXIST:
                    raise
                if no_wait or time.monotonic() >= deadline:
                    raise LockTimeout(
                        f"could not acquire lock for target {target_id!r} within {timeout}s",
                        {"lock": str(lock_path)},
                    )
                time.sleep(0.05)
        try:
            yield lock_path
        finally:
            if fd is not None:
                os.close(fd)
            try:
                os.unlink(lock_path)
            except OSError:
                pass
