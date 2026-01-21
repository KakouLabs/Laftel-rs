<h1 align="center">
  Laftel.rs
</h1>

<p align="center">
  <strong>Unofficial Rust Laftel.net API Wrapper</strong>
</p>

<p align="center">
  <a href="https://crates.io/crates/laftel-rs"><img src="https://img.shields.io/crates/v/laftel-rs.svg?style=flat-square&color=816BFF" alt="Crates.io"></a>
  <a href="https://docs.rs/laftel-rs"><img src="https://img.shields.io/badge/docs-latest-816BFF.svg?style=flat-square" alt="Docs.rs"></a>
  <img src="https://img.shields.io/crates/l/laftel-rs?color=816BFF&style=flat-square" alt="License">
</p>

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
laftel-rs = "0.1.0"
tokio = { version = "1.0", features = ["full"] } # For async support
```

## 🛠️ Usage

### Asynchronous (Default)

```rust
use laftel_rs::LaftelClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LaftelClient::new()?;
    
    // Search for anime
    let results = client.search_anime("전생슬").await?;
    
    if let Some(first) = results.first() {
        println!("Found: {} ({})", first.name, first.url());
        
        // Get detailed information
        let info = client.get_anime_info(first.id).await?;
        println!("Summary: {}", info.content);
    }
    
    Ok(())
}
```

### Synchronous (Blocking)

```rust
use laftel_rs::blocking::LaftelBlockingClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LaftelBlockingClient::new()?;
    
    let results = client.search_anime("전생슬")?;
    for result in results {
        println!("- {}", result.name);
    }
    
    Ok(())
}
```

## 📜 License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.

## 🤝 Acknowledgments

Inspired by the [original Python implementation](https://github.com/331leo/Laftel).
