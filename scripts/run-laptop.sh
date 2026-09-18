#!/usr/bin/env bash
set -euo pipefail

project_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if [[ ! -x "${project_dir}/target/release/linux-driver-booster" ]]; then
  printf '%s\n' "The app is not built yet. Run scripts/install-local.sh first." >&2
  exit 1
fi

exec "${project_dir}/target/release/linux-driver-booster" "$@"
