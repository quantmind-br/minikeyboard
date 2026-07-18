#!/usr/bin/env bash
# Install / uninstall the restricted Mini Keyboard udev rule.
# Replaces the insecure global MODE="0666" hidraw rule.
set -euo pipefail

RULE_NAME="70-minikeyboard.rules"
DEST="/etc/udev/rules.d/${RULE_NAME}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SOURCE="${SCRIPT_DIR}/../data/udev/${RULE_NAME}"

usage() {
  cat <<EOF
Usage: $0 [--uninstall]

Install restricted udev rule for Mini Keyboard (1189:8842) with TAG+=\"uaccess\".
Requires root.

  --uninstall   Remove the rule and reload udev
EOF
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

if [[ "$(id -u)" -ne 0 ]]; then
  echo "error: root required (run with sudo)" >&2
  exit 1
fi

if [[ "${1:-}" == "--uninstall" ]]; then
  if [[ -f "${DEST}" ]]; then
    rm -f "${DEST}"
    echo "removed ${DEST}"
  else
    echo "no rule at ${DEST}"
  fi
  udevadm control --reload-rules
  udevadm trigger --subsystem-match=hidraw
  echo "udev rules reloaded"
  exit 0
fi

if [[ ! -f "${SOURCE}" ]]; then
  echo "error: missing source rule: ${SOURCE}" >&2
  exit 1
fi

install -m 0644 "${SOURCE}" "${DEST}"
echo "installed ${DEST}"
udevadm control --reload-rules
udevadm trigger --subsystem-match=hidraw
echo "udev rules reloaded; reconnect the Mini Keyboard"
echo "rule content:"
cat "${DEST}"
