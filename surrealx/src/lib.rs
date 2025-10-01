//! # SurrealX - Extension Framework for SurrealDB
//!
//! SurrealX provides a clean API for extending SurrealDB with custom functions, events,
//! HTTP routes, and module composition while using the full SurrealDB server implementation.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use surrealx::{SurrealX, Module, ServerConfig};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let business = Module::new("business")
//!         .with_function("calculate_tax", |price: f64| async move {
//!             Ok(price * 0.15)
//!         });
//!
//!     SurrealX::new()
//!         .with_module(business)
//!         .serve(ServerConfig::default())
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

pub mod module;
pub mod functions;
pub mod events;
pub mod cache;
pub mod server;
pub mod error;

pub use module::Module;
pub use server::{SurrealX, ServerConfig};
pub use functions::{FunctionHandler, FunctionRegistry};
pub use events::{Event, EventListener, EventRegistry};
pub use cache::{CacheProvider, MemoryCacheProvider};
pub use error::{Error, Result};

#[cfg(feature = "redis-cache")]
pub use cache::RedisCacheProvider;

/// Re-exports for convenience
pub mod prelude {
    pub use crate::{
        SurrealX, ServerConfig, Module,
        FunctionHandler, FunctionRegistry,
        Event, EventListener, EventRegistry,
        CacheProvider, MemoryCacheProvider,
        Error, Result,
    };

    #[cfg(feature = "redis-cache")]
    pub use crate::RedisCacheProvider;
}
