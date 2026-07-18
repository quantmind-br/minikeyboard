#!/usr/bin/env python3
"""Identify the physical key order of the Mini Keyboard (1189:8842).

Interactively asks the user to press each physical control (knobs first,
then keys row by row, left to right) and records the input events each
press emits. Combined with a `dump-config` capture (wire position ->
configured action), the result reveals the wire index of every physical
slot so the app geometry can be corrected.

Usage:
    python3 tools/identify-layout.py [--skip-dump]

Requires read access to /dev/input/event* (root or `input` group).
No third-party dependencies.
"""

import argparse
import fcntl
import json
import os
import re
import select
import struct
import subprocess
import sys
import time

VID, PID = "1189", "8842"
EVENT_FORMAT = "llHHi"  # struct input_event (64-bit)
EVENT_SIZE = struct.calcsize(EVENT_FORMAT)
EVIOCGRAB = 0x40044590
EV_KEY, EV_REL, EV_MSC = 0x01, 0x02, 0x04
MSC_SCAN = 0x04

# Physical prompt order: two knobs on top, then 12 keys in 4 rows x 3 cols.
PROMPTS = [
    ("E1 girar-esq", "Gire o KNOB ESQUERDO (E1) para a ESQUERDA"),
    ("E1 clique", "PRESSIONE (clique) o KNOB ESQUERDO (E1)"),
    ("E1 girar-dir", "Gire o KNOB ESQUERDO (E1) para a DIREITA"),
    ("E2 girar-esq", "Gire o KNOB DIREITO (E2) para a ESQUERDA"),
    ("E2 clique", "PRESSIONE (clique) o KNOB DIREITO (E2)"),
    ("E2 girar-dir", "Gire o KNOB DIREITO (E2) para a DIREITA"),
] + [
    (
        f"K{i + 1} (linha {i // 3 + 1}, coluna {i % 3 + 1})",
        f"Pressione a tecla física LINHA {i // 3 + 1}, COLUNA {i % 3 + 1} "
        f"(contando do topo, esquerda para direita)",
    )
    for i in range(12)
]


def load_keycode_names():
    """Map keycode number -> KEY_* name from the kernel headers, if present."""
    names = {}
    for header in (
        "/usr/include/linux/input-event-codes.h",
        "/usr/include/linux/input.h",
    ):
        if not os.path.exists(header):
            continue
        for line in open(header, encoding="utf-8", errors="replace"):
            m = re.match(r"#define\s+(KEY_\w+|BTN_\w+)\s+(0x[0-9a-fA-F]+|\d+)", line)
            if m and m.group(1) not in ("KEY_MAX", "KEY_CNT"):
                code = int(m.group(2), 0)
                names.setdefault(code, m.group(1))
        if names:
            break
    return names


KEY_NAMES = load_keycode_names()
REL_NAMES = {0: "REL_X", 1: "REL_Y", 6: "REL_HWHEEL", 8: "REL_WHEEL", 11: "REL_WHEEL_HI_RES"}


def find_event_nodes():
    """All /dev/input/eventN nodes belonging to VID:PID."""
    nodes, block = [], ""
    for line in open("/proc/bus/input/devices", encoding="utf-8", errors="replace"):
        if line.startswith("I:"):
            block = line
        elif line.startswith("H:") and f"Vendor={VID} Product={PID}" in block:
            nodes += [f"/dev/input/{h}" for h in line.split() if h.startswith("event")]
    return nodes


def run_dump_config():
    """Wire position -> action label, from the dump-config helper."""
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    release = os.path.join(root, "target", "release", "dump-config")
    cmd = [release] if os.path.exists(release) else [
        "cargo", "run", "--quiet", "--release", "--bin", "dump-config"
    ]
    try:
        out = subprocess.run(
            cmd, cwd=root, capture_output=True, text=True, timeout=120
        )
    except (OSError, subprocess.TimeoutExpired) as e:
        print(f"  aviso: dump-config falhou ({e}); seguindo sem ele.")
        return None
    if out.returncode != 0:
        print(f"  aviso: dump-config retornou erro:\n{out.stderr.strip()}")
        return None
    layers, current = {}, None
    for line in out.stdout.splitlines():
        m = re.match(r"== layer (\d+)", line)
        if m:
            current = int(m.group(1))
            layers[current] = {}
            continue
        m = re.match(r"\s+pos\s+(\d+)\s+\(wire\s+\d+\):\s+(.+?)\s+raw\[", line)
        if m and current is not None:
            layers[current][int(m.group(1))] = m.group(2).strip()
    return layers


class Capture:
    def __init__(self, paths):
        self.fds = {}
        for p in paths:
            fd = os.open(p, os.O_RDONLY | os.O_NONBLOCK)
            fcntl.ioctl(fd, EVIOCGRAB, 1)  # keep presses out of the terminal
            self.fds[fd] = p

    def close(self):
        for fd in self.fds:
            try:
                fcntl.ioctl(fd, EVIOCGRAB, 0)
            finally:
                os.close(fd)

    def drain(self, seconds=0.4):
        """Discard queued events (key releases, auto-repeat, extra rotation)."""
        end = time.monotonic() + seconds
        while time.monotonic() < end:
            r, _, _ = select.select(list(self.fds), [], [], 0.05)
            for fd in r:
                try:
                    os.read(fd, EVENT_SIZE * 64)
                except BlockingIOError:
                    pass

    def _read_events(self, fd):
        try:
            data = os.read(fd, EVENT_SIZE * 64)
        except BlockingIOError:
            return []
        return [
            struct.unpack_from(EVENT_FORMAT, data, off)
            for off in range(0, len(data) - EVENT_SIZE + 1, EVENT_SIZE)
        ]

    def next_event(self, timeout=30.0):
        """Best event for one press: key press > wheel > raw scancode.

        MSC_SCAN is kept as fallback so keys whose action the kernel cannot
        map to a keycode (opaque/vendor actions) still register something.
        Returns None on timeout, "skip" when the user hits Enter.
        """
        end = time.monotonic() + timeout
        seen = []  # (etype, code, value, node)
        settle_until = None
        while time.monotonic() < end:
            wait = 0.05 if settle_until else 0.25
            r, _, _ = select.select(list(self.fds) + [sys.stdin], [], [], wait)
            if sys.stdin in r:
                sys.stdin.readline()
                return "skip"
            for fd in r:
                if fd is sys.stdin:
                    continue
                for _, _, etype, code, value in self._read_events(fd):
                    if etype in (EV_KEY, EV_REL, EV_MSC):
                        seen.append((etype, code, value, self.fds[fd]))
                        # First signal: keep collecting briefly so the KEY
                        # event that follows an MSC_SCAN is not lost.
                        settle_until = settle_until or time.monotonic() + 0.12
            if settle_until and time.monotonic() >= settle_until:
                break
        for etype, code, value, node in seen:
            if etype == EV_KEY and value == 1:
                return {"type": "key", "code": code,
                        "name": KEY_NAMES.get(code, f"KEY_{code}"), "node": node}
        for etype, code, value, node in seen:
            if etype == EV_REL and code in (6, 8, 11) and value != 0:
                return {"type": "rel", "code": code,
                        "name": REL_NAMES.get(code, f"REL_{code}"),
                        "value": value, "node": node}
        for etype, code, value, node in seen:
            if etype == EV_MSC and code == MSC_SCAN:
                return {"type": "scan", "code": value,
                        "name": f"MSC_SCAN 0x{value:06x}", "node": node}
        return None


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--skip-dump", action="store_true",
                    help="não executar dump-config antes da captura")
    ap.add_argument("-o", "--output", default="tools/physical-order-result.json")
    args = ap.parse_args()

    dump = None
    if not args.skip_dump:
        print("Lendo configuração atual do dispositivo (dump-config)...")
        dump = run_dump_config()
        if dump:
            print(f"  ok: {len(dump)} layer(s) lidas.")

    nodes = find_event_nodes()
    if not nodes:
        sys.exit(f"Nenhum /dev/input/event* para {VID}:{PID}. Teclado conectado?")
    try:
        cap = Capture(nodes)
    except PermissionError:
        sys.exit("Sem acesso a /dev/input. Rode com sudo ou entre no grupo 'input'.")

    print(f"\nNós capturados (grab exclusivo): {', '.join(nodes)}")
    print("As teclas NÃO digitarão no terminal durante a captura.")
    print("Enter = pular posição | Ctrl+C = abortar\n")

    results = []
    try:
        for label, prompt in PROMPTS:
            cap.drain()
            print(f">>> {label}: {prompt}")
            ev = cap.next_event()
            if ev == "skip":
                print("    (pulada)\n")
                results.append({"physical": label, "event": None})
                continue
            if ev is None:
                print("    (timeout — posição pulada)\n")
                results.append({"physical": label, "event": None})
                continue
            desc = ev["name"] + (f" ({ev['value']:+d})" if ev["type"] == "rel" else "")
            print(f"    capturado: {desc}  [{ev['node']}]\n")
            results.append({"physical": label, "event": ev})
    except KeyboardInterrupt:
        print("\nAbortado; salvando parcial.")
    finally:
        cap.close()

    report = {"vid": VID, "pid": PID, "captures": results, "dump_config": dump}
    with open(args.output, "w", encoding="utf-8") as f:
        json.dump(report, f, ensure_ascii=False, indent=2)

    print("\n=== Resumo (posição física -> evento) ===")
    for r in results:
        ev = r["event"]
        print(f"  {r['physical']:<28} {ev['name'] if ev else '—'}")
    if dump and 1 in dump:
        print("\n=== dump-config layer 1 (wire -> ação) ===")
        for pos, action in sorted(dump[1].items()):
            print(f"  wire {pos:2}  {action}")
    print(f"\nResultado salvo em {args.output} — envie esse arquivo para mapear a geometria.")


if __name__ == "__main__":
    main()
