# Linux Driver Booster

Linux Driver Booster is a safety-first Linux hardware and driver scanner. It uses the host distribution's native interfaces and is designed to grow into a Flatpak-compatible desktop utility without downloading arbitrary driver installers.

## Current implementation

- Detects the distribution from `/etc/os-release`.
- Detects the running kernel and architecture.
- Detects the preferred installed package manager (`apt`, `dnf`, `pacman`, or `zypper`).
- Scans PCI hardware through machine-readable `lspci` output.
- Reports loaded kernel drivers and loaded modules.
- Checks native package repositories for candidate updates without installing anything.
- Checks `fwupdmgr`, DKMS modules, installed kernels, and reboot indicators when available.
- Provides JSON output and `--history` diagnostics.
- Includes an asynchronous GTK4/libadwaita dashboard.

The scanner still does not install, remove, or modify packages. The typed operation contract and Polkit policy for a future host helper are in `src/security.rs` and `helper/`; no generic root command interface exists.

## Build and run

### Install on Pop!_OS or Ubuntu

This installs the application for your user and adds it to the desktop application menu:

```sh
chmod +x scripts/install-local.sh scripts/run-laptop.sh
./scripts/install-local.sh
```

Then open **Linux Driver Booster** from the application menu. To launch it from a terminal:

```sh
export PATH="$HOME/.local/bin:$PATH"
linux-driver-booster
```

The installer uses `sudo` only for installing build and hardware-detection packages. The application itself runs as your normal user.

```sh
cargo fmt --check
cargo test
cargo run -- --scan
cargo run -- --json
```

The `--scan` flag is accepted for compatibility with the planned CLI; a scan is the default operation. Use `--history` to inspect recorded update events. The JSON report is intended for integration tests and future UI layers.

To build the desktop UI on a system with GTK4/libadwaita development packages:

```sh
cargo run --features gtk-ui
```

## Supported distributions

The detection model has families for Debian/Ubuntu, Fedora, Arch, and openSUSE. Read-only package checks are implemented for APT, DNF, Pacman, and Zypper. Package output remains distribution-specific and is not treated as proof that a driver update is safe.

## Flatpak

The manifest is in `flatpak/com.linuxdriverbooster.DriverBooster.yml`. It uses the GNOME runtime and keeps host modification out of the application. The eventual privileged helper must be installed and reviewed separately; the UI must never gain a generic root command interface.

## Safety architecture

See `docs/security-architecture.md` and `helper/README.md`. Any future write operation must show a complete transaction preview, require explicit confirmation, use direct process arguments, and go through a narrowly defined Polkit-authorized host helper.

## Roadmap

1. Implement the separately installed helper and D-Bus transport after security review.
2. Add explicit transaction previews, package installation, failure logs, and reboot actions.
3. Add update history writes, rollback where the native backend supports it, and NVIDIA-specific recommendations.
4. Add USB/audio/storage adapters and broader distribution fixtures.
