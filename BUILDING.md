# Building and Testing SurrealX

## Prerequisites

- Rust 1.80.0 or later
- Cargo

## Quick Start

### 1. Build the Framework

```bash
# Build SurrealX framework
cargo build --package surrealx

# Or build entire workspace
cargo build --workspace
```

### 2. Run the Example

```bash
# Run the basic example demonstrating custom functions and events
cargo run --example basic
```

**Expected Output:**
```
🚀 SurrealX Extensions Loaded:
   Functions: ["ext::calculate_tax", "ext::format_currency"]
   Events: ["payments:*", "orders:*"]

✨ Framework ready for SurrealDB integration

📝 Next Steps:
   1. Complete SurrealDB server transformation
   2. Uncomment surrealdb-server dependency in Cargo.toml
   3. Implement ServerExtension integration
```

## Integration with SurrealDB

The framework is designed to integrate with a transformed SurrealDB server. The transformation has been prepared:

### 1. Download and Transform SurrealDB

```bash
# Integrate SurrealDB 2.3.10
./scripts/integrate.sh 2.3.10
```

This will:
- Download SurrealDB v2.3.10 source
- Move `src/` to `crates/server/` (transformation)
- Create server library API with extension points
- Mark transformation as complete

### 2. Enable Full Integration (Future)

To enable full SurrealDB server integration:

1. **Fix SurrealDB workspace dependencies** in `surrealdb/Cargo.toml`
   - The transformation creates a `crates/server/` crate
   - Workspace dependencies need to be properly configured

2. **Uncomment dependency** in `surrealx/Cargo.toml`:
   ```toml
   [dependencies]
   surrealdb-server = { path = "../surrealdb/crates/server" }
   ```

3. **Implement ServerExtension** in `surrealx/src/server.rs`:
   ```rust
   use surrealdb_server::{Server, ServerExtension};

   impl ServerExtension for SurrealXExtension {
       fn extend_router(&self, router: Router) -> Router {
           router.merge(self.router.clone())
       }

       async fn on_startup(&self) -> Result<()> {
           // Initialize extensions
           Ok(())
       }
   }
   ```

## Current Functionality

The framework currently demonstrates:

✅ **Module System**: Organize extensions with `Module::new()`
✅ **Custom Functions**: Register in `ext::` namespace
✅ **Event Listeners**: Pattern-based event handling (`orders:*`)
✅ **Cache Providers**: Memory and Redis support
✅ **Clean API**: Builder pattern with `with_*` methods

## Testing

```bash
# Run all tests
cargo test --workspace

# Run specific package tests
cargo test --package surrealx

# Check compilation
cargo check --workspace
```

## Project Structure

```
surrealx/
├── surrealx/              # Framework library
│   ├── src/
│   │   ├── lib.rs        # Main API
│   │   ├── module.rs     # Module system
│   │   ├── functions.rs  # Function registry
│   │   ├── events.rs     # Event system
│   │   ├── cache.rs      # Cache providers
│   │   ├── server.rs     # Server config
│   │   └── error.rs      # Error types
│   └── examples/
│       └── basic.rs      # Working example
├── integration/          # SurrealDB integration
│   ├── current.txt       # v2.0
│   └── transformations/
│       └── v2.0/         # Transformation scripts
├── scripts/
│   └── integrate.sh      # Integration automation
└── surrealdb/            # Transformed SurrealDB (gitignored)
    └── crates/server/    # Server as library
```

## Troubleshooting

### Cargo workspace errors
If you see workspace dependency errors, ensure:
- `surrealdb-server` dependency is commented out in `surrealx/Cargo.toml`
- Run `cargo clean` and rebuild

### Integration issues
The current setup demonstrates the framework without full SurrealDB compilation:
- Framework compiles and runs standalone
- Full integration requires resolving SurrealDB workspace dependencies
- See transformation scripts in `integration/transformations/v2.0/`

## Next Development Steps

1. **Complete SurrealDB Workspace**: Resolve all workspace dependencies in transformed SurrealDB
2. **Function Hooks**: Implement actual SQL function interception
3. **Event Emission**: Connect to SurrealDB transaction hooks
4. **HTTP Integration**: Merge routers with SurrealDB server
5. **System Functions**: Implement `sx::*` functions (emit, cache, webhook)
