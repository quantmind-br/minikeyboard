# Flatpak packaging helpers

## Manifest

`br.com.quantmind.MiniKeyboard.Devel.json` targets **GNOME Platform/SDK 50**.

Host udev installation is still required for hardware access (`--device=all` alone is not enough for unprivileged hidraw without the `uaccess` rule).

## Refresh `cargo-sources.json`

The checked-in `cargo-sources.json` vendors every crate pinned by `Cargo.lock` for
Flatpak's offline build. Regenerate it whenever the lockfile changes:

```bash
flatpak-cargo-generator.py Cargo.lock -o build-aux/cargo-sources.json
```

The manifest includes the generated file directly in its module `sources` array.

If runtime 50 is unavailable, try runtime 49 only after proving the same locked
crate set compiles with `flatpak-builder`.
