//! Module system for organizing extensions

use std::sync::Arc;
use axum::Router;
use serde_json::Value;
use crate::functions::{FunctionHandler, SimpleFunctionHandler};
use crate::events::{EventListener, SimpleEventListener};
use crate::error::Result;

/// A module encapsulating related functionality
pub struct Module {
    name: String,
    functions: Vec<(String, Arc<dyn FunctionHandler>)>,
    listeners: Vec<(String, Arc<dyn EventListener>)>,
    routes: Vec<(&'static str, Router)>,
}

impl Module {
    /// Create a new module
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            functions: Vec::new(),
            listeners: Vec::new(),
            routes: Vec::new(),
        }
    }

    /// Add a custom function to the module
    pub fn with_function<F, Fut>(mut self, name: impl Into<String>, handler: F) -> Self
    where
        F: Fn(Vec<Value>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<Value>> + Send + 'static,
    {
        let handler = SimpleFunctionHandler::new(move |args| Box::pin(handler(args)));
        self.functions.push((name.into(), Arc::new(handler)));
        self
    }

    /// Add a raw function handler to the module
    pub fn with_raw_function<H>(mut self, name: impl Into<String>, handler: H) -> Self
    where
        H: FunctionHandler + 'static,
    {
        self.functions.push((name.into(), Arc::new(handler)));
        self
    }

    /// Add an event listener to the module
    pub fn with_listener<F, Fut>(mut self, pattern: impl Into<String>, handler: F) -> Self
    where
        F: Fn(crate::events::Event) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + 'static,
    {
        let handler = SimpleEventListener::new(move |event| Box::pin(handler(event)));
        self.listeners.push((pattern.into(), Arc::new(handler)));
        self
    }

    /// Add a raw event listener to the module
    pub fn with_raw_listener<L>(mut self, pattern: impl Into<String>, listener: L) -> Self
    where
        L: EventListener + 'static,
    {
        self.listeners.push((pattern.into(), Arc::new(listener)));
        self
    }

    /// Add an HTTP route to the module
    pub fn with_route(mut self, path: &'static str, router: Router) -> Self {
        self.routes.push((path, router));
        self
    }

    /// Get module name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get all functions
    pub fn functions(&self) -> &[(String, Arc<dyn FunctionHandler>)] {
        &self.functions
    }

    /// Get all listeners
    pub fn listeners(&self) -> &[(String, Arc<dyn EventListener>)] {
        &self.listeners
    }

    /// Get all routes
    pub fn routes(&self) -> &[(&'static str, Router)] {
        &self.routes
    }
}
