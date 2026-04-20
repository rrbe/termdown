#!/usr/bin/env bash
# termdown uninstaller
#
# Removes the installed binary and deletes the config directory (~/.termdown).
# Never invokes sudo; if the binary's location is not writable, prints a clear
# hint and exits.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/rrbe/termdown/master/uninstall.sh | bash
#
# Environment variables:
#   TERMDOWN_INSTALL_DIR   location of the binary (default: auto-detect via `command -v`)
#   TERMDOWN_KEEP_CONFIG   set to 1 to keep ~/.termdown (default: remove it)

set -euo pipefail

REPO="rrbe/termdown"

info() { printf '\033[1;32m==>\033[0m %s\n' "$*"; }
err()  { printf '\033[1;31merror:\033[0m %s\n' "$*" >&2; }

if [ -n "${TERMDOWN_INSTALL_DIR:-}" ]; then
  BIN="${TERMDOWN_INSTALL_DIR%/}/termdown"
else
  BIN="$(command -v termdown 2>/dev/null || true)"
fi

if [ -z "$BIN" ] || [ ! -e "$BIN" ]; then
  info "termdown binary not found — nothing to remove"
else
  info "Removing ${BIN}"
  if ! rm -f "$BIN" 2>/dev/null; then
    err "Cannot remove ${BIN} (permission denied)."
    cat >&2 <<EOF

Hint — retry with sudo:

  curl -fsSL https://raw.githubusercontent.com/${REPO}/master/uninstall.sh | sudo bash

EOF
    exit 1
  fi
fi

if [ "${TERMDOWN_KEEP_CONFIG:-0}" = "1" ]; then
  info "Keeping config directory (~/.termdown)"
else
  if [ -d "$HOME/.termdown" ]; then
    info "Removing config directory (~/.termdown)"
    rm -rf "$HOME/.termdown"
  fi
fi

info "Uninstall complete"
