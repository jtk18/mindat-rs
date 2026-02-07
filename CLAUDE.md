# Claude Code Instructions

This file contains project-specific instructions for Claude Code.

## Git Commit Authorship

When creating commits in this repository:

1. **Author**: Commits should be authored by the repository owner (jtk18)
2. **Co-authored-by**: Add Claude as a co-author using the trailer:
   ```
   Co-authored-by: Claude <noreply@anthropic.com>
   ```

### Commit Format Example

```bash
git commit -m "$(cat <<'EOF'
Your commit message here

Co-authored-by: Claude <noreply@anthropic.com>
EOF
)"
```

## Project Context

- **mindat-rs**: Rust client library for the Mindat mineralogical database API
- **GUI**: Optional Tauri-based GUI in `gui/` directory
- **Rust version**: 1.85+
- **Edition**: 2024

## Development Notes

- The main library is at the workspace root
- The GUI is a separate workspace member in `gui/src-tauri/`
- GUI requires system GTK libraries on Linux
- IMA minerals endpoint works without authentication

## Development Environment Setup

### Linux (Ubuntu/Debian)

To build the GUI on Linux, install the required system dependencies:

```bash
sudo apt-get update && sudo apt-get install -y \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    libappindicator3-dev \
    librsvg2-dev \
    patchelf
```

### macOS

Xcode command line tools should be sufficient:

```bash
xcode-select --install
```

### Windows

Install the WebView2 runtime (usually pre-installed on Windows 10+).

## Building the GUI

```bash
# Build the GUI
cargo build -p mindat-gui

# Run in development mode
cargo run -p mindat-gui

# Build only the library (without GUI)
cargo build --lib
```

## Testing

```bash
# Run library tests
cargo test

# Run with authentication (required for most endpoints)
MINDAT_API_KEY=your_key cargo test
```

## Tauri 2.0 GUI Development

### Parameter Naming Convention

**IMPORTANT**: Tauri 2.0 automatically converts Rust snake_case parameter names to JavaScript camelCase when invoking commands.

| Rust function parameter | JavaScript invoke key |
|------------------------|----------------------|
| `radius_km` | `radiusKm` |
| `name_contains` | `nameContains` |
| `page_size` | `pageSize` |
| `include_elements` | `includeElements` |
| `exclude_elements` | `excludeElements` |

Example:
```rust
// Rust command
#[tauri::command]
async fn search_localities_by_gps(
    latitude: f64,
    longitude: f64,
    radius_km: f64,  // snake_case in Rust
) -> Result<...>
```

```javascript
// JavaScript invoke - use camelCase!
await invoke('search_localities_by_gps', {
    latitude: lat,
    longitude: lon,
    radiusKm: radiusKm  // camelCase in JS
});
```

### WebView2 Caching on Windows

Windows WebView2 aggressively caches HTML/JS files. If changes aren't appearing after rebuild:

```powershell
# Clear WebView2 cache
Remove-Item -Recurse -Force "$env:LOCALAPPDATA\org.mindat.explorer" -ErrorAction SilentlyContinue
```

### Debug Logging

- **Rust backend**: Uses `debug_log!` macro (only in debug builds), outputs to stderr
- **JavaScript frontend**: Uses `debug()` function, outputs to browser DevTools console (F12)
- DevTools is enabled via `"devtools": true` in tauri.conf.json

### Mindat API Notes

- Country names use abbreviations: "USA" not "United States", "UK" not "United Kingdom"
- API returns limited results per page (~10-20), use pagination for more
- GPS locality search requires country or name filter to avoid timeout
- Address geocoding uses Nominatim/OpenStreetMap (free, no API key)

## MCP Server for Claude Desktop

The `mcp-server/` directory contains an MCP (Model Context Protocol) server that exposes the mindat-rs library to Claude Desktop and other MCP clients.

### Building the MCP Server

```bash
# Build the MCP server
cargo build -p mindat-mcp --release

# The binary will be at target/release/mindat-mcp
```

### Claude Desktop Configuration

Add the following to your Claude Desktop config file:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
**Linux**: `~/.config/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "mindat": {
      "command": "/path/to/mindat-mcp",
      "env": {
        "MINDAT_API_KEY": "your-api-key-here"
      }
    }
  }
}
```

Replace `/path/to/mindat-mcp` with the actual path to the built binary.

### Available MCP Tools

The MCP server exposes these tools to Claude:

| Tool | Description | Requires API Key |
|------|-------------|------------------|
| `search_minerals` | Search minerals by name | Yes |
| `search_ima_minerals` | Search IMA-approved minerals | **No** |
| `get_mineral` | Get mineral details by ID | Yes |
| `search_by_elements` | Find minerals by chemical elements | Yes |
| `quick_search` | Fast autocomplete search | Yes |
| `search_localities` | Search mineral localities | Yes |
| `search_localities_by_gps` | Find localities near coordinates | Yes |
| `get_locality` | Get locality details by ID | Yes |
| `list_countries` | List all countries | Yes |
| `get_dana_groups` | Get Dana classification groups | Yes |
| `get_strunz_classes` | Get Strunz classification classes | Yes |
| `get_photo_count` | Get database photo statistics | Yes |

### API Key

Get a free API key at https://api.mindat.org/ - the `search_ima_minerals` tool works without authentication for basic mineral searches.
