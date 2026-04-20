#!/usr/bin/env bash
# termdown installer
#
# Downloads the prebuilt binary for your platform from GitHub Releases,
# verifies its SHA-256 checksum, and installs it. Never invokes sudo; if the
# install directory is not writable, prints a clear hint and exits.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/rrbe/termdown/master/install.sh | bash
#
# Environment variables:
#   TERMDOWN_VERSION       release tag to install (default: latest)
#   TERMDOWN_INSTALL_DIR   install directory (default: /usr/local/bin)

set -euo pipefail

REPO="rrbe/termdown"
VERSION="${TERMDOWN_VERSION:-latest}"
INSTALL_DIR="${TERMDOWN_INSTALL_DIR:-/usr/local/bin}"

info() { printf '\033[1;32m==>\033[0m %s\n' "$*"; }
warn() { printf '\033[1;33mwarning:\033[0m %s\n' "$*" >&2; }
err()  { printf '\033[1;31merror:\033[0m %s\n' "$*" >&2; }

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Darwin) OS_STR=apple-darwin ;;
  Linux)  OS_STR=unknown-linux-gnu ;;
  *)
    err "Unsupported OS: $OS (only macOS and Linux are supported by this script)."
    err "For Windows, download the archive manually from https://github.com/${REPO}/releases."
    exit 1
    ;;
esac

case "$ARCH" in
  x86_64|amd64)  ARCH_STR=x86_64 ;;
  arm64|aarch64) ARCH_STR=aarch64 ;;
  *) err "Unsupported architecture: $ARCH"; exit 1 ;;
esac

TARGET="${ARCH_STR}-${OS_STR}"
ARCHIVE="termdown-${TARGET}.tar.gz"

if [ "$VERSION" = "latest" ]; then
  BASE="https://github.com/${REPO}/releases/latest/download"
else
  BASE="https://github.com/${REPO}/releases/download/${VERSION}"
fi

info "Installing termdown (${VERSION}, ${TARGET})"

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

info "Downloading ${ARCHIVE}"
curl -fsSL -o "${TMP}/${ARCHIVE}"    "${BASE}/${ARCHIVE}"
curl -fsSL -o "${TMP}/SHA256SUMS"    "${BASE}/SHA256SUMS"

info "Verifying checksum"
if command -v shasum >/dev/null 2>&1; then
  CHECK_CMD="shasum -a 256 -c -"
elif command -v sha256sum >/dev/null 2>&1; then
  CHECK_CMD="sha256sum -c -"
else
  err "Neither 'shasum' nor 'sha256sum' is available; cannot verify the download."
  exit 1
fi
(cd "$TMP" && grep " ${ARCHIVE}\$" SHA256SUMS | $CHECK_CMD >/dev/null)

info "Extracting"
tar -xzf "${TMP}/${ARCHIVE}" -C "$TMP"

mkdir -p "$INSTALL_DIR" 2>/dev/null || true
DEST="${INSTALL_DIR%/}/termdown"
if ! install -m 0755 "${TMP}/termdown" "$DEST" 2>/dev/null; then
  err "Cannot write to ${INSTALL_DIR} (permission denied)."
  cat >&2 <<EOF

Hint — pick one:

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
