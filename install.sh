#!/usr/bin/env bash
# termdown installer compatibility wrapper.
#
# New releases ship cargo-dist archives. Older releases only have the original
# tar.gz archives plus SHA256SUMS. Keep this script in the repo so README can
# point at a stable URL across the transition while preserving the existing
# TERMDOWN_INSTALL_DIR behavior.

set -euo pipefail

REPO="rrbe/termdown"
BASE="https://github.com/${REPO}/releases/latest/download"

info() { printf '\033[1;32m==>\033[0m %s\n' "$*"; }
warn() { printf '\033[1;33mwarning:\033[0m %s\n' "$*" >&2; }
err()  { printf '\033[1;31merror:\033[0m %s\n' "$*" >&2; }

INSTALL_DIR="${TERMDOWN_INSTALL_DIR:-/usr/local/bin}"

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Darwin) OS_STR=apple-darwin ;;
  Linux)  OS_STR=unknown-linux-gnu ;;
  *)
    err "Unsupported OS: $OS (only macOS and Linux are supported by this script)."
    err "For Windows, use cargo install or download an installer from https://github.com/${REPO}/releases."
    exit 1
    ;;
esac

case "$ARCH" in
  x86_64|amd64)  ARCH_STR=x86_64 ;;
  arm64|aarch64) ARCH_STR=aarch64 ;;
  *) err "Unsupported architecture: $ARCH"; exit 1 ;;
esac

TARGET="${ARCH_STR}-${OS_STR}"
DIST_ARCHIVE="termdown-${TARGET}.tar.xz"
LEGACY_ARCHIVE="termdown-${TARGET}.tar.gz"

info "Installing termdown (${TARGET})"

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

if command -v shasum >/dev/null 2>&1; then
  CHECK_CMD="shasum -a 256 -c -"
elif command -v sha256sum >/dev/null 2>&1; then
  CHECK_CMD="sha256sum -c -"
else
  err "Neither 'shasum' nor 'sha256sum' is available; cannot verify the download."
  exit 1
fi

info "Checking for ${DIST_ARCHIVE}"
if curl --proto '=https' --tlsv1.2 -fsSL -o "${TMP}/${DIST_ARCHIVE}" "${BASE}/${DIST_ARCHIVE}" 2>/dev/null &&
   curl --proto '=https' --tlsv1.2 -fsSL -o "${TMP}/${DIST_ARCHIVE}.sha256" "${BASE}/${DIST_ARCHIVE}.sha256" 2>/dev/null; then
  info "Verifying checksum"
  (cd "$TMP" && $CHECK_CMD "${DIST_ARCHIVE}.sha256" >/dev/null)
  info "Extracting"
  tar -xf "${TMP}/${DIST_ARCHIVE}" -C "$TMP"
  BIN="${TMP}/termdown-${TARGET}/termdown"
else
  warn "latest release does not include cargo-dist archives yet; using legacy archive install"
  info "Downloading ${LEGACY_ARCHIVE}"
  curl -fsSL -o "${TMP}/${LEGACY_ARCHIVE}" "${BASE}/${LEGACY_ARCHIVE}"
  curl -fsSL -o "${TMP}/SHA256SUMS" "${BASE}/SHA256SUMS"
  info "Verifying checksum"
  (cd "$TMP" && grep " ${LEGACY_ARCHIVE}\$" SHA256SUMS | $CHECK_CMD >/dev/null)
  info "Extracting"
  tar -xzf "${TMP}/${LEGACY_ARCHIVE}" -C "$TMP"
  BIN="${TMP}/termdown"
fi

mkdir -p "$INSTALL_DIR" 2>/dev/null || true
DEST="${INSTALL_DIR%/}/termdown"
if ! install -m 0755 "$BIN" "$DEST" 2>/dev/null; then
  err "Cannot write to ${INSTALL_DIR} (permission denied)."
  cat >&2 <<EOF

Hint - pick one:

  # Install globally with sudo
  curl -fsSL https://raw.githubusercontent.com/${REPO}/master/install.sh | sudo bash

  # Install to a user-owned directory instead
  curl -fsSL https://raw.githubusercontent.com/${REPO}/master/install.sh \\
    | TERMDOWN_INSTALL_DIR="\$HOME/.local/bin" bash

EOF
  exit 1
fi

info "Installed termdown to ${DEST}"

case ":${PATH}:" in
  *:"${INSTALL_DIR%/}":*) ;;
  *)
    warn "${INSTALL_DIR} is not in your PATH."
    printf '  Add this to your shell config (then restart your shell):\n\n    export PATH="%s:$PATH"\n\n' "${INSTALL_DIR%/}" >&2
    ;;
esac

"$DEST" --version
