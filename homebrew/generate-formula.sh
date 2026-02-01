#!/bin/bash
# Generate Homebrew formula with correct SHA256 hashes
# Usage: ./generate-formula.sh v0.1.0

set -euo pipefail

VERSION="${1:-}"
if [[ -z "$VERSION" ]]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 v0.1.0"
    exit 1
fi

# Strip 'v' prefix for formula version
FORMULA_VERSION="${VERSION#v}"

REPO="christopherwxyz/remix-mcp"
BASE_URL="https://github.com/${REPO}/releases/download/${VERSION}"

# Download and compute SHA256 for each platform
echo "Fetching SHA256 hashes for ${VERSION}..."

get_sha256() {
    local url="$1"
    curl -sL "$url" | shasum -a 256 | cut -d' ' -f1
}

SHA_ARM64=$(get_sha256 "${BASE_URL}/remix-mcp-aarch64-apple-darwin.tar.gz")
SHA_X86_64=$(get_sha256 "${BASE_URL}/remix-mcp-x86_64-apple-darwin.tar.gz")
SHA_LINUX_X86_64=$(get_sha256 "${BASE_URL}/remix-mcp-x86_64-unknown-linux-gnu.tar.gz")

echo "  macOS ARM64:  ${SHA_ARM64}"
echo "  macOS x86_64: ${SHA_X86_64}"
echo "  Linux x86_64: ${SHA_LINUX_X86_64}"

cat > Formula/remix-mcp.rb << EOF
# typed: false
# frozen_string_literal: true

class RemixMcp < Formula
  desc "MCP server for controlling Ableton Live via OSC"
  homepage "https://github.com/${REPO}"
  version "${FORMULA_VERSION}"
  license "MIT"

  on_macos do
    on_arm do
      url "${BASE_URL}/remix-mcp-aarch64-apple-darwin.tar.gz"
      sha256 "${SHA_ARM64}"
    end
    on_intel do
      url "${BASE_URL}/remix-mcp-x86_64-apple-darwin.tar.gz"
      sha256 "${SHA_X86_64}"
    end
  end

  on_linux do
    on_intel do
      url "${BASE_URL}/remix-mcp-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "${SHA_LINUX_X86_64}"
    end
  end

  def install
    bin.install "remix-mcp"
  end

  def caveats
    <<~EOS
      To install the AbletonOSC Remote Script, run:
        remix-mcp install

      Then restart Ableton Live and enable AbletonOSC in:
        Preferences > Link/Tempo/MIDI > Control Surface
    EOS
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/remix-mcp --version")
  end
end
EOF

echo "Formula written to Formula/remix-mcp.rb"
