//! Custom function registry and handlers

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use serde_json::Value;
use crate::error::Result;

/// Handler for custom SQL functions
#[async_trait]
pub trait FunctionHandler: Send + Sync {
    /// Execute the function with given arguments
    async fn call(&self, args: Vec<Value>) -> Result<Value>;
}

/// Simple function handler using async closures
pub struct SimpleFunctionHandler<F>
where
    F: Fn(Vec<Value>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Value>> + Send>> + Send + Sync,
{
    handler: F,
}

impl<F> SimpleFunctionHandler<F>
where
    F: Fn(Vec<Value>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Value>> + Send>> + Send + Sync,
{
    pub fn new(handler: F) -> Self {
        Self { handler }
    }
}

#[async_trait]
impl<F> FunctionHandler for SimpleFunctionHandler<F>
where
    F: Fn(Vec<Value>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Value>> + Send>> + Send + Sync,
{
    async fn call(&self, args: Vec<Value>) -> Result<Value> {
        (self.handler)(args).await
    }
}

/// Registry for custom functions
#[derive(Clone)]
pub struct FunctionRegistry {
    functions: Arc<HashMap<String, Arc<dyn FunctionHandler>>>,
}

impl FunctionRegistry {
    pub fn new() -> Self {
        Self {
            functions: Arc::new(HashMap::new()),
        }
    }

    /// Register a new function
    pub fn register<H>(&mut self, name: impl Into<String>, handler: H)
    where
        H: FunctionHandler + 'static,
    {
        let functions = Arc::get_mut(&mut self.functions)
            .expect("Cannot modify registry with existing references");
        functions.insert(name.into(), Arc::new(handler));
    }

    /// Register a function that's already wrapped in Arc
    pub fn register_arc(&mut self, name: impl Into<String>, handler: Arc<dyn FunctionHandler>) {
        let functions = Arc::get_mut(&mut self.functions)
            .expect("Cannot modify registry with existing references");
        functions.insert(name.into(), handler);
    }

    /// Get a function handler by name
    pub fn get(&self, name: &str) -> Option<Arc<dyn FunctionHandler>> {
        self.functions.get(name).cloned()
    }

    /// Check if a function exists
    pub fn contains(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    /// List all registered function names
    pub fn list(&self) -> Vec<String> {
        self.functions.keys().cloned().collect()
    }
}

impl Default for FunctionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
