# SurrealX

Extension framework for SurrealDB - Add custom SQL functions, events, HTTP routes, and modules while using the full SurrealDB server.

## Features

- **Custom SQL Functions**: Define functions in `ext::*` and `sx::*` namespaces
- **Event System**: React to database changes with event listeners
- **HTTP Routes**: Add custom API endpoints to the SurrealDB server
- **Module Composition**: Organize extensions into reusable modules
- **Dual Cache Providers**: SurrealDB memory (self-contained) or Redis (distributed)
- **Full SurrealDB Compatibility**: Works with Surrealist and all SurrealDB tools
- **Version Mirroring**: SurrealX v2.3.10 = SurrealDB v2.3.10

## Quick Start

```rust
use surrealx::{SurrealX, Module, ServerConfig};

#[tokio::main]
async fn main() -> Result<()> {
    // Define custom function
    async fn calculate_tax(price: f64) -> f64 {
        price * 0.15
    }

    // Create module
    let business = Module::new("business")
        .with_function("calculate_tax", calculate_tax)
        .with_listener("orders:*", |event| async move {
            println!("Order event: {:?}", event);
        })
        .with_route("/webhooks/payment", handle_payment);

    // Start server with extensions
    SurrealX::new()
        .with_module(business)
        .serve(ServerConfig::default())
        .await?;

    Ok(())
}
```

## SQL Usage

```sql
-- Use custom function
SELECT ext::calculate_tax(100.0) AS tax;

-- Use SurrealX system functions
SELECT sx::emit('payment:completed', { order_id: 123 });
SELECT sx::cache::set('key', 'value', 3600);
```

## Installation

### From crates.io

```toml
[dependencies]
surrealx = "2.3.10"
```

### From GitHub

```toml
[dependencies]
surrealx = { git = "https://github.com/rzerve/surrealx", tag = "v2.3.10" }
```

## Architecture

SurrealX uses source transformation to convert SurrealDB from a binary-only application to a reusable library:

1. **Download**: Fetch SurrealDB source for the desired version
2. **Transform**: Move `src/` to `crates/server/` with extension points
3. **Extend**: Add custom functionality through the SurrealX API
4. **Deploy**: Run as a single binary with full SurrealDB capabilities

## Version Strategy

- **SurrealX version mirrors SurrealDB version**: v2.3.10 → v2.3.10
- **Transformation versions are separate**: v2.0 works for all 2.x
- **Update transformation only for breaking changes**: Not every minor version

## Integration

```bash
# Integrate SurrealDB v2.3.10
./scripts/integrate.sh 2.3.10

# Build workspace
cargo build --workspace

# Run tests
cargo test --workspace
```

## Documentation

- [Architecture Guide](docs/ARCHITECTURE.md)
- [Integration Guide](docs/INTEGRATION_GUIDE.md)
- [API Reference](https://docs.rs/surrealx)

## License

MIT OR Apache-2.0
