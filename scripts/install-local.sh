#!/usr/bin/env bash
set -euo pipefail

project_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
install_dir="${HOME}/.local/share/linux-driver-booster"
bin_dir="${HOME}/.local/bin"
desktop_dir="${HOME}/.local/share/applications"

if ! command -v apt-get >/dev/null 2>&1; then
  printf '%s\n' "This installer currently supports Ubuntu, Pop!_OS, and Debian-family systems." >&2
  exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
  printf '%s\n' "Rust is required. Install it from https://rustup.rs/ and run this installer again." >&2
  exit 1
fi

sudo apt-get update
sudo apt-get install --yes build-essential pkg-config libgtk-4-dev libadwaita-1-dev pciutils usbutils fwupd dkms

mkdir -p "${install_dir}" "${bin_dir}" "${desktop_dir}"
cargo build --release --features gtk-ui --manifest-path "${project_dir}/Cargo.toml"
install -Dm755 "${project_dir}/target/release/linux-driver-booster" "${install_dir}/linux-driver-booster"

cat > "${bin_dir}/linux-driver-booster" <<EOF
#!/usr/bin/env bash
exec "${install_dir}/linux-driver-booster" "\$@"
EOF
chmod 755 "${bin_dir}/linux-driver-booster"

cat > "${desktop_dir}/com.linuxdriverbooster.DriverBooster.desktop" <<EOF
[Desktop Entry]
Name=Linux Driver Booster
Comment=Inspect Linux hardware, drivers, firmware, and updates
Exec=${bin_dir}/linux-driver-booster
Icon=computer-symbolic
Terminal=false
Type=Application
Categories=System;Utility;
Keywords=drivers;hardware;firmware;kernel;
StartupNotify=true
EOF

printf '\nInstalled Linux Driver Booster for this user.\n'
printf 'Run it from the application menu or with: linux-driver-booster\n'
if [[ ":${PATH}:" != *":${bin_dir}:"* ]]; then
  printf 'Add %s to PATH for the terminal command:\n' "${bin_dir}"
  printf '  export PATH="$HOME/.local/bin:$PATH"\n'
fi
