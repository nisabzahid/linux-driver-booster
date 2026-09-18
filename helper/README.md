# Host Helper Contract

The Flatpak application is deliberately unable to install packages directly. A separately installed host helper may implement the typed operations in `src/security.rs` over a narrowly scoped system D-Bus interface.

Required rules:

- Expose only named operations: refresh repositories, install or update an approved package, install a selected fwupd update, and repair one selected DKMS module.
- Validate every package, device, and module identifier before invoking a backend.
- Use `std::process::Command` with fixed executable names and separate arguments. Never invoke `sh -c` and never expose a `run_as_root` method.
- Require a Polkit authorization for every mutating operation.
- Return a structured preview/result with affected packages, errors, and reboot state.
- Never accept filesystem paths, command strings, or arbitrary environment variables from the Flatpak.

The MVP does not ship or install this helper. This document is the implementation contract for the next packaging phase.
