# mindat-rs

A Rust client library for the [Mindat API](https://api.mindat.org/).

[![Crates.io](https://img.shields.io/crates/v/mindat-rs.svg)](https://crates.io/crates/mindat-rs)
[![Documentation](https://docs.rs/mindat-rs/badge.svg)](https://docs.rs/mindat-rs)
[![CI](https://github.com/jtk18/mindat-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/jtk18/mindat-rs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Mindat is the world's largest open database of minerals, rocks, meteorites, and the localities where they come from. This crate provides a type-safe, async interface to access mineralogical data.

## Features

- Full coverage of the Mindat API endpoints
- Strongly-typed request builders and response models
- Async/await support using tokio
- Pagination helpers
- Comprehensive error handling
- **Optional GUI application** built with Tauri (supports desktop and mobile)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mindat-rs = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use mindat_rs::{MindatClient, GeomaterialsQuery, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client with your API token
    let client = MindatClient::new("your-api-token");

    // Search for quartz
    let query = GeomaterialsQuery::new()
        .name("quartz")
        .ima_approved(true);

    let minerals = client.geomaterials(query).await?;

    for mineral in minerals.results {
        println!("{}: {:?}", mineral.id, mineral.name);
    }

    Ok(())
}
```

## Authentication

Most API endpoints require authentication with a Mindat API token. You can obtain a token from your [Mindat account settings](https://www.mindat.org/).

Some endpoints (like `minerals_ima`) can be accessed without authentication:

```rust
use mindat_rs::{MindatClient, ImaMineralsQuery};

let client = MindatClient::anonymous();
let minerals = client.minerals_ima(ImaMineralsQuery::new()).await?;
```

## Examples

### Search for minerals by element composition

```rust
use mindat_rs::{MindatClient, GeomaterialsQuery};

let client = MindatClient::new("your-token");

// Find minerals containing copper and sulfur
let query = GeomaterialsQuery::new()
    .with_elements("Cu,S")
    .ima_approved(true)
    .page_size(50);

let minerals = client.geomaterials(query).await?;
```

### Filter by physical properties

```rust
use mindat_rs::{MindatClient, GeomaterialsQuery, CrystalSystem};

let client = MindatClient::new("your-token");

// Find hard, dense minerals in the cubic system
let query = GeomaterialsQuery::new()
    .crystal_systems(vec![CrystalSystem::Isometric])
    .hardness_range(7.0, 10.0)
    .density_range(5.0, 20.0);

let minerals = client.geomaterials(query).await?;
```

### Search localities

```rust
use mindat_rs::{MindatClient, LocalitiesQuery};

let client = MindatClient::new("your-token");

// Find gold localities in Brazil
let query = LocalitiesQuery::new()
    .country("Brazil")
    .with_elements("Au");

let localities = client.localities(query).await?;
```

### Browse IMA minerals

```rust
use mindat_rs::{MindatClient, ImaMineralsQuery};

// IMA list doesn't require authentication
let client = MindatClient::anonymous();

let query = ImaMineralsQuery::new()
    .search("diamond")
    .page_size(10);

let minerals = client.minerals_ima(query).await?;
```

### Pagination

```rust
use mindat_rs::{MindatClient, GeomaterialsQuery};

let client = MindatClient::new("your-token");

// Get first page
let query = GeomaterialsQuery::new().page(1).page_size(100);
let page1 = client.geomaterials(query).await?;

println!("Total minerals: {:?}", page1.count);

// Check if there are more pages
if page1.has_next() {
    let query = GeomaterialsQuery::new().page(2).page_size(100);
    let page2 = client.geomaterials(query).await?;
}
```

### Custom client configuration

```rust
use mindat_rs::MindatClient;
use std::time::Duration;

let client = MindatClient::builder()
    .token("your-token")
    .timeout(Duration::from_secs(60))
    .build()?;
```

## Available Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `countries()` | GET | List all countries |
| `country(id)` | GET | Get a specific country |
| `geomaterials(query)` | GET | Search minerals with filters |
| `geomaterial(id)` | GET | Get a specific geomaterial |
| `geomaterial_varieties(id)` | GET | Get varieties of a geomaterial |
| `geomaterials_search(q, size)` | GET | Quick search for geomaterials |
| `localities(query)` | GET | Search localities with filters |
| `locality(id)` | GET | Get a specific locality |
| `locality_ages(page)` | GET | List locality ages |
| `locality_age(id)` | GET | Get a specific age |
| `locality_statuses(page)` | GET | List locality statuses |
| `locality_status(id)` | GET | Get a specific status |
| `locality_types(page)` | GET | List locality types |
| `locality_type(id)` | GET | Get a specific type |
| `minerals_ima(query)` | GET | List IMA-approved minerals |
| `mineral_ima(id)` | GET | Get a specific IMA mineral |
| `dana8_groups()` | GET | Dana 8th ed. classification groups |
| `dana8_subgroups()` | GET | Dana 8th ed. classification subgroups |
| `dana8(id)` | GET | Get a specific Dana classification |
| `strunz10_classes()` | GET | Nickel-Strunz 10th ed. classes |
| `strunz10_subclasses()` | GET | Nickel-Strunz 10th ed. subclasses |
| `strunz10_families()` | GET | Nickel-Strunz 10th ed. families |
| `strunz10(id)` | GET | Get a specific Strunz classification |

## Error Handling

The library provides detailed error types:

```rust
use mindat_rs::{MindatClient, MindatError, GeomaterialsQuery};

let client = MindatClient::new("your-token");
let query = GeomaterialsQuery::new().name("quartz");

match client.geomaterials(query).await {
    Ok(minerals) => println!("Found {} minerals", minerals.results.len()),
    Err(MindatError::AuthenticationRequired) => {
        eprintln!("Invalid or missing API token");
    }
    Err(MindatError::RateLimited) => {
        eprintln!("Too many requests, please wait");
    }
    Err(MindatError::NotFound(msg)) => {
        eprintln!("Resource not found: {}", msg);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## GUI Application (Optional)

This crate includes an optional GUI application built with [Tauri](https://tauri.app/) for testing and exploring the Mindat API. The GUI provides a user-friendly interface to:

- Search minerals by name, elements, or properties
- Browse IMA-approved minerals (no authentication required)
- Explore localities and countries
- View classification systems (Dana-8, Strunz-10)
- Get detailed mineral information

### Prerequisites

**Linux:**
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
  libssl-dev libayatana-appindicator3-dev librsvg2-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel openssl-devel curl wget file \
  libappindicator-gtk3-devel librsvg2-devel
```

**macOS:**
```bash
xcode-select --install
```

**Windows:**
- Install [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- Install [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

### Building the GUI

```bash
# From the repository root
cd gui/src-tauri
cargo build --release

# Or run in development mode
cargo run
```

### Running the GUI

After building, the executable will be at:
- Linux/macOS: `target/release/mindat-gui`
- Windows: `target\release\mindat-gui.exe`

### Using the GUI

1. Launch the application
2. Enter your Mindat API token (from [mindat.org](https://www.mindat.org/)) and click "Connect"
3. Use the sidebar to select different API endpoints
4. The "IMA Minerals" endpoint works without authentication

### Mobile Support (iOS/Android)

The GUI is built with Tauri 2.0, which supports mobile platforms. To build for iOS:

```bash
# Install Tauri CLI
cargo install tauri-cli

# Initialize iOS (requires Xcode)
cd gui/src-tauri
cargo tauri ios init

# Build for iOS
cargo tauri ios build
```

For Android:
```bash
# Initialize Android (requires Android Studio)
cargo tauri android init

# Build for Android
cargo tauri android build
```

See the [Tauri Mobile Guide](https://v2.tauri.app/start/prerequisites/) for detailed setup instructions.

## Related Projects

- [OpenMindat](https://github.com/quexiang/OpenMindat) - R package for Mindat API
- [mindat_api_test](https://github.com/ChuBL/How-to-Use-Mindat-API) - Python examples

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

- [Mindat.org](https://www.mindat.org/) for providing the mineralogical database and API
- The OpenMindat R package for implementation reference
