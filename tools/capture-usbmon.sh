#!/usr/bin/env bash
# Capture Mini Keyboard HID traffic for golden-vector review.
# Writes timestamped .pcapng + metadata JSON outside source control by default.
set -euo pipefail

BUS=""
DEVICE=""
OPERATION="capture"
OUTPUT_DIR="${HOME}/.local/share/minikeyboard/captures"

usage() {
  cat <<EOF
Usage: $0 --bus N --device M [--operation NAME] [--output-dir DIR]

Capture USB traffic for bus/device via tshark/usbmon.
Requires root or CAP_NET_ADMIN / access to usbmon.

  --bus N           USB bus number (from lsusb -t)
  --device M        Device number on that bus
  --operation NAME  Label stored in metadata (default: capture)
  --output-dir DIR  Output directory (default: ~/.local/share/minikeyboard/captures)
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --bus) BUS="${2:-}"; shift 2 ;;
    --device) DEVICE="${2:-}"; shift 2 ;;
    --operation) OPERATION="${2:-}"; shift 2 ;;
    --output-dir) OUTPUT_DIR="${2:-}"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown arg: $1" >&2; usage; exit 1 ;;
  esac
done

if [[ -z "${BUS}" || -z "${DEVICE}" ]]; then
  echo "error: --bus and --device required" >&2
  usage
  exit 1
fi

if ! command -v tshark >/dev/null 2>&1; then
  echo "error: tshark not found (install wireshark-cli)" >&2
  exit 1
fi

# Mount debugfs only when already permitted.
if [[ ! -d /sys/kernel/debug/usb ]]; then
  if [[ -w /sys/kernel/debug ]]; then
    mount -t debugfs none /sys/kernel/debug 2>/dev/null || true
  fi
fi

if [[ ! -d /sys/kernel/debug/usb ]]; then
  echo "error: /sys/kernel/debug/usb unavailable."
  echo "run as root or: sudo mount -t debugfs none /sys/kernel/debug"
  exit 1
fi

mkdir -p "${OUTPUT_DIR}"
TS="$(date -u +%Y%m%dT%H%M%SZ)"
BASE="${OUTPUT_DIR}/${TS}-${OPERATION}-bus${BUS}-dev${DEVICE}"
PCAP="${BASE}.pcapng"
META="${BASE}.json"

echo "capturing usbmon bus ${BUS} device ${DEVICE} → ${PCAP}"
echo "press Ctrl+C to stop"

FILTER="usb.bus_id == ${BUS} && usb.device_address == ${DEVICE}"

cat > "${META}" <<EOF
{
  "timestamp_utc": "${TS}",
  "operation": "${OPERATION}",
  "bus": ${BUS},
  "device": ${DEVICE},
  "filter": "${FILTER}",
  "pcapng": "$(basename "${PCAP}")",
  "host": "$(hostname)",
  "note": "Review before importing with import-vector; reject unrelated USB addresses."
}
EOF

tshark -i "usbmon${BUS}" -f "" -Y "${FILTER}" -w "${PCAP}" || {
  echo "tshark failed — try: sudo $0 --bus ${BUS} --device ${DEVICE} --operation ${OPERATION}"
  exit 1
}
