"""Clean-room headless Ghidra reverse-engineering CLI.

Public state machine: initialized | analyzed | enriched | decompiled |
validated | failed. JSON is the canonical artifact format. stdlib-only.
"""

__version__ = "1.0.0"
SCHEMA_VERSION = 1
