#!/usr/bin/env bash
# Meson helper: cargo build then copy binary to @OUTPUT@.
set -euo pipefail

OUTPUT="${1:?output path}"
shift
BIN_NAME="${1:?binary name}"
shift
PROFILE_DIR="${1:?profile dir name (release|debug)}"
shift

cargo build "$@"
TARGET_DIR=""
# Parse --target-dir from remaining cargo options.
args=("$@")
for i in "${!args[@]}"; do
  if [[ "${args[$i]}" == "--target-dir" ]]; then
    TARGET_DIR="${args[$((i + 1))]}"
    break
  fi
done
if [[ -z "${TARGET_DIR}" ]]; then
  TARGET_DIR="target"
fi

cp -f "${TARGET_DIR}/${PROFILE_DIR}/${BIN_NAME}" "${OUTPUT}"
