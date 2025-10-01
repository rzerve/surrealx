#!/bin/bash
set -euo pipefail

SURREALDB_DIR="${1:-surrealdb}"
VERSION="${2:-}"

echo "🔧 SurrealX Transformation v2.0 (for SurrealDB 2.x)"
echo "📦 Transforming: $SURREALDB_DIR"
[[ -n "$VERSION" ]] && echo "📌 Version: $VERSION"

# Validate SurrealDB directory
if [[ ! -d "$SURREALDB_DIR/src" ]]; then
    echo "❌ Error: $SURREALDB_DIR/src not found"
    exit 1
fi

# 1. Create server crate structure
echo "📁 Creating server crate structure..."
mkdir -p "$SURREALDB_DIR/crates/server/src"

# 2. Move server implementation from src/ to crates/server/src/
echo "📦 Moving server implementation..."
if [[ -d "$SURREALDB_DIR/src/cli" ]]; then
    mv "$SURREALDB_DIR/src/cli" "$SURREALDB_DIR/crates/server/src/"
fi

if [[ -d "$SURREALDB_DIR/src/net" ]]; then
    mv "$SURREALDB_DIR/src/net" "$SURREALDB_DIR/crates/server/src/"
fi

if [[ -d "$SURREALDB_DIR/src/rpc" ]]; then
    mv "$SURREALDB_DIR/src/rpc" "$SURREALDB_DIR/crates/server/src/"
fi

# 3. Create server library API
echo "🔌 Creating library API..."
cat > "$SURREALDB_DIR/crates/server/src/lib.rs" << 'EOF'
//! SurrealDB Server Library
//!
//! This crate exposes the SurrealDB server implementation as a reusable library.
//! It provides extension points for frameworks like SurrealX to add custom functionality.

pub mod cli;
pub mod net;
pub mod rpc;

pub use net::Server;
pub use cli::Config as ServerConfig;

use std::future::Future;
use anyhow::Result;

/// Extension trait for customizing the SurrealDB server
pub trait ServerExtension: Send + Sync {
    /// Extend the HTTP router with custom routes
    fn extend_router(&self, router: axum::Router) -> axum::Router {
        router
    }

    /// Hook called when server starts
    fn on_startup(&self) -> impl Future<Output = Result<()>> + Send {
        async { Ok(()) }
    }

    /// Hook called when server shuts down
    fn on_shutdown(&self) -> impl Future<Output = Result<()>> + Send {
        async { Ok(()) }
    }
}
EOF

# 4. Create server Cargo.toml
echo "📝 Creating server Cargo.toml..."
cat > "$SURREALDB_DIR/crates/server/Cargo.toml" << EOF
[package]
name = "surrealdb-server"
version = "${VERSION:-2.0.0}"
edition = "2021"
description = "SurrealDB server implementation as a library"
license = "BSL-1.1"

[dependencies]
surrealdb = { path = "../.." }
surrealdb-core = { path = "../../core" }
axum = "0.7"
tokio = { version = "1", features = ["full"] }
anyhow = "1"
futures = "0.3"

[dependencies.clap]
version = "4"
features = ["derive", "env"]
EOF

# 5. Update workspace Cargo.toml to include server crate
echo "🔧 Updating workspace..."
if [[ -f "$SURREALDB_DIR/Cargo.toml" ]]; then
    # Add server to workspace members if not already present
    if ! grep -q 'crates/server' "$SURREALDB_DIR/Cargo.toml"; then
        # This is a simplified approach - in production, use proper TOML parser
        sed -i.bak '/members = \[/,/\]/ s/\]/    "crates\/server",\n]/' "$SURREALDB_DIR/Cargo.toml"
        rm "$SURREALDB_DIR/Cargo.toml.bak" 2>/dev/null || true
    fi
fi

# 6. Create .surrealx-transformed marker
echo "✅ Creating transformation marker..."
echo "v2.0" > "$SURREALDB_DIR/.surrealx-transformed"
[[ -n "$VERSION" ]] && echo "$VERSION" >> "$SURREALDB_DIR/.surrealx-transformed"

echo "✨ Transformation complete!"
echo ""
echo "Next steps:"
echo "  1. Review changes in $SURREALDB_DIR"
echo "  2. Update SurrealX Cargo.toml to depend on surrealdb-server"
echo "  3. Run: cargo build --workspace"
