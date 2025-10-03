//! Server configuration and main API

use std::sync::Arc;
use axum::Router;
use crate::module::Module;
use crate::functions::FunctionRegistry;
use crate::events::EventRegistry;
use crate::cache::{CacheProvider, MemoryCacheProvider};
use crate::error::Result;

/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub bind_addr: String,
    pub data_path: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_addr: "127.0.0.1:8000".to_string(),
            data_path: None,
        }
    }
}

/// Main SurrealX API
pub struct SurrealX {
    modules: Vec<Module>,
    function_registry: FunctionRegistry,
    event_registry: EventRegistry,
    cache_provider: Arc<dyn CacheProvider>,
}

impl SurrealX {
    /// Create a new SurrealX instance
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            function_registry: FunctionRegistry::new(),
            event_registry: EventRegistry::new(),
            cache_provider: Arc::new(MemoryCacheProvider::new()),
        }
    }

    /// Add a module
    pub fn with_module(mut self, module: Module) -> Self {
        self.modules.push(module);
        self
    }

    /// Set cache provider
    pub fn with_cache<C>(mut self, provider: C) -> Self
    where
        C: CacheProvider + 'static,
    {
        self.cache_provider = Arc::new(provider);
        self
    }

    /// Build the extension system
    pub async fn build(mut self) -> Result<BuiltSurrealX> {
        // Register all functions from modules
        for module in &self.modules {
            for (name, handler) in module.functions() {
                // Functions in modules are registered with ext:: prefix
                let full_name = format!("ext::{}", name);
                self.function_registry.register_arc(full_name, handler.clone());
            }
        }

        // Register all event listeners from modules
        for module in &self.modules {
            for (pattern, listener) in module.listeners() {
                self.event_registry.register_arc(pattern, listener.clone()).await;
            }
        }

        let router = self.build_router();

        Ok(BuiltSurrealX {
            function_registry: self.function_registry,
            event_registry: self.event_registry,
            cache_provider: self.cache_provider,
            router,
        })
    }

    /// Serve the SurrealX server
    pub async fn serve(self, _config: ServerConfig) -> Result<()> {
        let built = self.build().await?;

        println!("🚀 SurrealX Extensions Loaded:");
        println!("   Functions: {:?}", built.function_registry.list());
        println!("   Events: {:?}", built.event_registry.patterns().await);
        println!();
        println!("✨ Framework ready for SurrealDB integration");
        println!();
        println!("📝 Next Steps:");
        println!("   1. Complete SurrealDB server transformation");
        println!("   2. Uncomment surrealdb-server dependency in Cargo.toml");
        println!("   3. Implement ServerExtension integration");

        Ok(())
    }

    fn build_router(&self) -> Router {
        let mut router = Router::new();

        // Add routes from modules
        for module in &self.modules {
            for (path, module_router) in module.routes() {
                router = router.nest(path, module_router.clone());
            }
        }

        router
    }
}

impl Default for SurrealX {
    fn default() -> Self {
        Self::new()
    }
}

/// Built SurrealX instance with all extensions registered
pub struct BuiltSurrealX {
    pub function_registry: FunctionRegistry,
    pub event_registry: EventRegistry,
    pub cache_provider: Arc<dyn CacheProvider>,
    pub router: Router,
}
