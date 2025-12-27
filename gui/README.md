# Mindat Explorer GUI

A desktop and mobile application for exploring the Mindat mineralogical database, built with [Tauri](https://tauri.app/) and the `mindat-rs` library.

## Features

- **Mineral Search**: Search minerals by name using wildcards
- **IMA Minerals**: Browse IMA-approved minerals (no API key required)
- **Element Search**: Find minerals by chemical composition
- **Locality Browser**: Explore mineral localities by country
- **Classification Systems**: View Dana-8 and Strunz-10 classifications
- **Detailed Views**: Get comprehensive mineral information

## Development

### Prerequisites

1. **Rust 1.85+** - Install via [rustup](https://rustup.rs/)
2. **System dependencies** - See main [README](../README.md#prerequisites)

### Running in Development

```bash
cd src-tauri
cargo run
```

### Building for Release

```bash
cd src-tauri
cargo build --release
```

The executable will be in `target/release/mindat-gui`.

## Project Structure

```
gui/
├── src/                    # Frontend (HTML/CSS/JS)
│   └── index.html          # Main application UI
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs         # Application entry point
│   │   └── lib.rs          # Tauri commands
│   ├── capabilities/       # Tauri permission configuration
│   ├── Cargo.toml          # Rust dependencies
│   └── tauri.conf.json     # Tauri configuration
└── README.md               # This file
```

## Available Commands

The GUI exposes these Tauri commands that interface with the `mindat-rs` library:

| Command | Description |
|---------|-------------|
| `set_api_token` | Configure API authentication |
| `search_minerals` | Search minerals by name |
| `search_ima_minerals` | Search IMA-approved minerals (public) |
| `get_mineral` | Get mineral details by ID |
| `search_by_elements` | Find minerals by element composition |
| `list_countries` | List all countries |
| `search_localities` | Search localities |
| `get_dana8_groups` | Get Dana-8 classification |
| `get_strunz10_classes` | Get Strunz-10 classification |

## Mobile Development

### iOS

```bash
cargo install tauri-cli
cargo tauri ios init
cargo tauri ios dev    # Development
cargo tauri ios build  # Production
```

### Android

```bash
cargo install tauri-cli
cargo tauri android init
cargo tauri android dev    # Development
cargo tauri android build  # Production
```

## License

MIT - See [LICENSE](../LICENSE)
