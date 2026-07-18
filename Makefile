# Mini Keyboard — developer convenience targets.
# Cargo is the build path; Meson is the distro install path (SPEC packaging).

.PHONY: all build install uninstall clean test clippy check run udev-install udev-uninstall help

CARGO       ?= cargo
MESON       ?= meson
NINJA       ?= ninja
SUDO        ?= sudo
PREFIX      ?= /usr/local
DESTDIR     ?=
BUILDDIR    ?= build
PROFILE     ?= release
CARGO_FLAGS ?=

ifeq ($(PROFILE),release)
  CARGO_PROFILE_FLAG := --release
else
  CARGO_PROFILE_FLAG :=
endif

BIN := target/$(PROFILE)/minikeyboard
# Absolute build dir so elevated ninja/meson still find it after sudo.
BUILDDIR_ABS := $(abspath $(BUILDDIR))

# Elevate only the file-install / uninstall step when writing the real prefix.
# Staging (DESTDIR set) and already-root sessions stay unprivileged.
NEED_ROOT := $(if $(DESTDIR),,$(if $(filter 0,$(shell id -u 2>/dev/null || echo 1)),,1))

all: build

## build — native release (or PROFILE=debug) binary via Cargo
build:
	$(CARGO) build $(CARGO_PROFILE_FLAG) $(CARGO_FLAGS)
	@test -x $(BIN)
	@file $(BIN)
	@echo "built $(BIN)"

## install — Meson setup/compile as user; install step prompts sudo when needed
##   make install                              # PREFIX=/usr/local, prompts password
##   PREFIX=/usr make install                  # system prefix, prompts password
##   DESTDIR=$$PWD/stage make install          # staged tree, no sudo
install:
	@if [ -f $(BUILDDIR)/build.ninja ]; then \
		$(MESON) setup $(BUILDDIR) --prefix='$(PREFIX)' -Dprofile=$(PROFILE) --reconfigure; \
	else \
		$(MESON) setup $(BUILDDIR) --prefix='$(PREFIX)' -Dprofile=$(PROFILE); \
	fi
	$(MESON) compile -C $(BUILDDIR)
ifeq ($(NEED_ROOT),1)
	@echo "installing to $(PREFIX) (sudo password may be required)"
	$(SUDO) env DESTDIR='$(DESTDIR)' $(MESON) install -C '$(BUILDDIR_ABS)'
else
	DESTDIR='$(DESTDIR)' $(MESON) install -C $(BUILDDIR)
endif
	@echo "installed under $(DESTDIR)$(PREFIX) (see meson summary)"

## uninstall — ninja uninstall of last Meson install record; sudo when needed
uninstall:
	@if [ ! -f $(BUILDDIR)/meson-logs/install-log.txt ]; then \
		echo "error: no meson install log in $(BUILDDIR); run 'make install' first" >&2; \
		exit 1; \
	fi
ifeq ($(NEED_ROOT),1)
	@echo "uninstalling from last install log (sudo password may be required)"
	$(SUDO) $(NINJA) -C '$(BUILDDIR_ABS)' uninstall
else
	$(NINJA) -C $(BUILDDIR) uninstall
endif
	@echo "note: if the udev rule was also installed by setup-hid-permissions.sh, run: make udev-uninstall"

## udev-install — restricted 1189:8842 uaccess rule (prompts sudo)
udev-install:
	$(SUDO) ./linux/setup-hid-permissions.sh

## udev-uninstall — remove project udev rule (prompts sudo)
udev-uninstall:
	$(SUDO) ./linux/setup-hid-permissions.sh --uninstall

## test / clippy / check
test:
	$(CARGO) test --all-targets $(CARGO_FLAGS)

clippy:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

check: test clippy

## run — release binary (override with PROFILE=debug)
run: build
	./$(BIN)

## clean
clean:
	$(CARGO) clean
	rm -rf $(BUILDDIR) stage

help:
	@echo "targets:"
	@echo "  make / make build   cargo build --$(PROFILE)  → $(BIN)"
	@echo "  make install        meson install to PREFIX=$(PREFIX) (prompts sudo unless DESTDIR set)"
	@echo "  make uninstall      ninja uninstall last install log (prompts sudo when needed)"
	@echo "  make test           cargo test --all-targets"
	@echo "  make clippy         cargo clippy -D warnings"
	@echo "  make check          test + clippy"
	@echo "  make run            build then run"
	@echo "  make udev-install   install restricted udev rule (prompts sudo)"
	@echo "  make udev-uninstall remove udev rule (prompts sudo)"
	@echo "  make clean          cargo clean + rm build/ stage/"
	@echo ""
	@echo "vars: PROFILE=release|debug  PREFIX=/usr/local  DESTDIR=  SUDO=sudo  CARGO_FLAGS="
