"""Atomic JSON artifact I/O with schema stamping.

JSON is canonical. Every write is atomic (temp file + os.replace on the same
filesystem). Records are stamped with schema_version, target_id, source, and
observed_at so provenance is uniform across artifact groups.
"""
from __future__ import annotations

import json
import os
import tempfile
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from . import SCHEMA_VERSION


def now_iso() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def stamp(record: dict[str, Any], target_id: str, source: str) -> dict[str, Any]:
    """Return a copy of record with canonical provenance fields prepended."""
    stamped = {
        "schema_version": SCHEMA_VERSION,
        "target_id": target_id,
        "source": source,
        "observed_at": now_iso(),
    }
    stamped.update(record)
    return stamped


def write_json(path: Path, obj: Any) -> Path:
    """Atomically write obj as pretty JSON to path (parents created)."""
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, tmp = tempfile.mkstemp(dir=str(path.parent), prefix=".tmp-", suffix=".json")
    try:
        with os.fdopen(fd, "w", encoding="utf-8") as f:
            json.dump(obj, f, indent=2, sort_keys=False)
            f.write("\n")
            f.flush()
            os.fsync(f.fileno())
        os.replace(tmp, path)
    except BaseException:
        try:
            os.unlink(tmp)
        except OSError:
            pass
        raise
    return path


def read_json(path: Path) -> Any:
    return json.loads(Path(path).read_text(encoding="utf-8"))


def append_jsonl(path: Path, record: dict[str, Any]) -> Path:
    """Append one JSON record as a single line.

    JSONL streams (execution-log.jsonl, runtime events.jsonl) cannot use the
    temp-file + os.replace whole-document swap, since that rewrites rather than
    appends. Correctness relies on the CALLER HOLDING THE TARGET LOCK
    (Workspace.lock) so writers are serialized -- that is the actual atomicity
    guarantee, not any per-write kernel property. O_APPEND still guarantees each
    write is positioned at EOF, and we loop until all bytes are written so a
    short write cannot truncate the record; fsync durably persists it.
    """
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    line = (json.dumps(record, sort_keys=False) + "\n").encode("utf-8")
    fd = os.open(str(path), os.O_CREAT | os.O_WRONLY | os.O_APPEND, 0o644)
    try:
        mv = memoryview(line)
        while mv:
            written = os.write(fd, mv)
            mv = mv[written:]
        os.fsync(fd)
    finally:
        os.close(fd)
    return path


def relpath(path: Path, workspace: Path) -> str:
    """Path relative to workspace for artifact listings, else absolute."""
    try:
        return str(Path(path).resolve().relative_to(Path(workspace).resolve()))
    except ValueError:
        return str(Path(path).resolve())
