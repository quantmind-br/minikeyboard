"""Doctor install-guidance tests.

Simulate Linux/macOS/Windows and isolated absence of each dependency; assert the
guidance names an official URL, manager-appropriate commands, a manual route,
and a post-install verify command. No installation is performed.
"""
from __future__ import annotations

import pytest

from ghidra_skill import doctor


def _guidance_for(dep_substr, guidance):
    for g in guidance:
        if dep_substr.lower() in g["dependency"].lower():
            return g
    return None


def test_missing_ghidra_guidance(monkeypatch):
    # Force Ghidra absent regardless of host.
    monkeypatch.setattr(doctor, "discover_ghidra",
                        lambda home=None: {"found": False, "home": None,
                                           "analyze_headless": None, "version": None,
                                           "pyghidra_run": False})
    monkeypatch.delenv("GHIDRA_HOME", raising=False)
    monkeypatch.delenv("GHIDRA_INSTALL_DIR", raising=False)
    report = doctor.run_doctor()
    g = _guidance_for("Ghidra", report["guidance"])
    assert g is not None
    assert g["official_url"].startswith("https://")
    assert g["manual_route"]
    assert g["verify_command"]
    assert report["static_ready"] is False


@pytest.mark.parametrize("os_kind,managers", [
    ("linux", ["apt-get", "dnf", "pacman"]),
    ("macos", ["brew"]),
    ("windows", ["winget", "choco"]),
])
def test_per_os_manager_commands(monkeypatch, os_kind, managers):
    monkeypatch.setattr(doctor, "_os_kind", lambda: os_kind)
    monkeypatch.setattr(doctor, "_detect_pkg_managers", lambda k: managers)
    g = doctor.guidance_jdk(os_kind, managers)
    # every detected manager yields a command, and a manual route always exists
    assert len(g["install_commands"]) == len(managers)
    assert g["manual_route"] and g["official_url"].startswith("https://")


def test_manual_route_when_no_manager(monkeypatch):
    # No package managers detected -> still a manual route, no commands.
    g = doctor.guidance_binutils("linux", [])
    assert g["install_commands"] == []
    assert g["manual_route"]


def test_frida_absent_guidance(monkeypatch):
    monkeypatch.setattr(doctor, "check_frida",
                        lambda: {"cli": False, "version": None,
                                 "python_module": False, "found": False})
    report = doctor.run_doctor(ghidra_home=None)
    g = _guidance_for("Frida", report["guidance"])
    assert g and "pip install frida-tools" in " ".join(g["install_commands"])


def test_pyghidra_launcher_vs_library(monkeypatch):
    # launcher present, library absent -> not ready, guidance present
    monkeypatch.setattr(doctor, "discover_ghidra",
                        lambda home=None: {"found": True, "home": "/opt/ghidra",
                                           "analyze_headless": "/opt/ghidra/support/analyzeHeadless",
                                           "version": "12.1.2", "pyghidra_run": True})
    monkeypatch.setattr(doctor, "check_pyghidra",
                        lambda gh: {"launcher": True, "library": False, "ready": False})
    report = doctor.run_doctor()
    g = _guidance_for("PyGhidra", report["guidance"])
    assert g and any("pip install pyghidra" in c for c in g["install_commands"])
    assert any("--no-index" in c for c in g["install_commands"])  # offline route
