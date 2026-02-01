![remix-mcp banner](https://github.com/christopherwxyz/remix-mcp/blob/main/banner.png?raw=true)

A Rust MCP server that enables AI assistants to control Ableton Live via OSC.

[![CI](https://github.com/christopherwxyz/remix-mcp/actions/workflows/ci.yml/badge.svg)](https://github.com/christopherwxyz/remix-mcp/actions/workflows/ci.yml)

## Features

- **266 tools** for comprehensive Ableton Live control
- **Real-time OSC** via [AbletonOSC](https://github.com/ideoforms/AbletonOSC)
- **Auto-installer** for the AbletonOSC Remote Script
- **Cross-platform** (macOS, Windows, Linux)

## Quick Start

### 1. Install remix-mcp

**Via uv** (recommended, requires [uv](https://docs.astral.sh/uv/)):
```bash
uvx remix-mcp install
```

**Download binary** from [Releases](https://github.com/christopherwxyz/remix-mcp/releases)

**Build from source:**
```bash
git clone --recursive https://github.com/christopherwxyz/remix-mcp
cd remix-mcp
cargo build --release
```

### 2. Install AbletonOSC

```bash
remix-mcp install
```

Then in Ableton Live:
1. Restart Ableton Live
2. Open **Preferences** (Cmd+, / Ctrl+,)
3. Go to **Link/Tempo/MIDI**
4. Under **Control Surface**, select **AbletonOSC**

You should see: `AbletonOSC: Listening for OSC on port 11000`

### 3. Add MCP Server

**Claude Desktop:**
1. Open Claude Desktop Settings > Developer
2. Click Edit Config to open `claude_desktop_config.json`
3. Add the server configuration:
```json
{
  "mcpServers": {
    "ableton": {
      "command": "uvx",
      "args": ["remix-mcp"]
    }
  }
}
```
4. Restart Claude Desktop

**Claude Code:**
```bash
claude mcp add ableton -- uvx remix-mcp
```

## Usage

Ask Claude things like:

- "Create a MIDI track with Drift and add reverb"
- "Set the tempo to 128 BPM"
- "Create a 4-bar drum pattern"
- "Add an arpeggiator to the selected track"
- "Search the browser for bass sounds"

See [docs/examples.md](docs/examples.md) for more examples.

## Tools

| Category | Count | Examples |
|----------|-------|----------|
| Transport | 10 | `play`, `stop`, `record`, `set_tempo` |
| Tracks | 59 | `create_midi_track`, `set_track_volume`, `arm_track` |
| Clips | 65 | `fire_clip`, `create_clip`, `add_midi_notes` |
| Scenes | 19 | `fire_scene`, `create_scene`, `duplicate_scene` |
| Devices | 10 | `list_devices`, `set_device_parameter` |
| Song | 56 | `undo`, `redo`, `set_loop`, `get_quantization` |
| View | 8 | `select_track`, `select_clip`, `select_device` |
| Cue Points | 5 | `list_cue_points`, `jump_to_cue_point` |
| Browser | 29 | `load_instrument`, `load_audio_effect`, `search_browser` |
| Application | 4 | `get_version`, `get_application_view` |
| MIDI Map | 1 | `get_midi_map_addresses` |

## Architecture

```
Claude/Client <--stdio/JSON-RPC--> remix-mcp <--UDP/OSC--> AbletonOSC <--> Ableton Live
```

## CLI

```bash
remix-mcp serve              # Start MCP server (default)
remix-mcp install            # Install AbletonOSC Remote Script
remix-mcp install --force    # Reinstall
remix-mcp status             # Check installation
```

## Troubleshooting

**No sound from MIDI tracks?**
Samplers need samples loaded. Use `load_default_instrument` to load Drift (a synth that works immediately).

**Connection timeout?**
1. Check Ableton Live is running
2. Verify AbletonOSC is enabled in Preferences > Link/Tempo/MIDI
3. Ensure ports 11000/11001 are free

**AbletonOSC not in Control Surface list?**
1. Run `remix-mcp install`
2. Restart Ableton Live

## Development

See [CLAUDE.md](CLAUDE.md) for development guidelines.

## License

MIT

## Acknowledgments

- [AbletonOSC](https://github.com/ideoforms/AbletonOSC) by Daniel Jones
