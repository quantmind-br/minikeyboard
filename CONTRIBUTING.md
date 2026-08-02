# Contributing to MiniKeyboard

Contributions are welcome, especially reproducible device captures, Linux
packaging improvements, accessibility fixes, and protocol documentation.

## Development setup

Install Rust stable, GTK 4, libadwaita, hidapi, and libudev development files.
On Arch Linux:

```bash
sudo pacman -S rust gtk4 libadwaita hidapi pkgconf
```

Before opening a pull request, run:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
```

Use the mock scenarios described in [README.md](README.md) for UI work that
does not require hardware.

## Device support

Do not enable writes for a new VID/PID based only on visual similarity or a
single USB capture. A device becomes writable only after reversible
write/readback behavior is documented and reviewed. Never include proprietary
vendor binaries, firmware, personal input captures, or device serial numbers.

Report security-sensitive protocol findings according to
[SECURITY.md](SECURITY.md).
