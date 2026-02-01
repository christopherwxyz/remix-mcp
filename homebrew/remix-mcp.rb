# typed: false
# frozen_string_literal: true

class RemixMcp < Formula
  desc "MCP server for controlling Ableton Live via OSC"
  homepage "https://github.com/christopherwxyz/remix-mcp"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/christopherwxyz/remix-mcp/releases/download/v#{version}/remix-mcp-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_ARM64"
    end
    on_intel do
      url "https://github.com/christopherwxyz/remix-mcp/releases/download/v#{version}/remix-mcp-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_X86_64"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/christopherwxyz/remix-mcp/releases/download/v#{version}/remix-mcp-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER_SHA256_LINUX_ARM64"
    end
    on_intel do
      url "https://github.com/christopherwxyz/remix-mcp/releases/download/v#{version}/remix-mcp-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER_SHA256_LINUX_X86_64"
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
