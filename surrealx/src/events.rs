//! Event system for database change notifications

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::RwLock;
use crate::error::Result;

/// Database event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum EventType {
    Create,
    Update,
    Delete,
    Custom(String),
}

/// Event emitted when database changes occur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Event type
    pub event_type: EventType,
    /// Table name (e.g., "orders")
    pub table: String,
    /// Record ID
    pub record_id: Option<String>,
    /// Event data
    pub data: Value,
    /// Timestamp
    pub timestamp: i64,
}

impl Event {
    /// Create a new event
    pub fn new(event_type: EventType, table: impl Into<String>, data: Value) -> Self {
        Self {
            event_type,
            table: table.into(),
            record_id: None,
            data,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    /// Set record ID
    pub fn with_record_id(mut self, id: impl Into<String>) -> Self {
        self.record_id = Some(id.into());
        self
    }

    /// Get the pattern for this event (e.g., "orders:123" or "orders:*")
    pub fn pattern(&self) -> String {
        if let Some(id) = &self.record_id {
            format!("{}:{}", self.table, id)
        } else {
            format!("{}:*", self.table)
        }
    }
}

/// Listener for events
#[async_trait]
pub trait EventListener: Send + Sync {
    /// Handle an event
    async fn on_event(&self, event: Event) -> Result<()>;
}

/// Simple event listener using async closures
pub struct SimpleEventListener<F>
where
    F: Fn(Event) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send>> + Send + Sync,
{
    handler: F,
}

impl<F> SimpleEventListener<F>
where
    F: Fn(Event) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send>> + Send + Sync,
{
    pub fn new(handler: F) -> Self {
        Self { handler }
    }
}

#[async_trait]
impl<F> EventListener for SimpleEventListener<F>
where
    F: Fn(Event) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send>> + Send + Sync,
{
    async fn on_event(&self, event: Event) -> Result<()> {
        (self.handler)(event).await
    }
}

/// Registry for event listeners
#[derive(Clone)]
pub struct EventRegistry {
    listeners: Arc<RwLock<HashMap<String, Vec<Arc<dyn EventListener>>>>>,
}

impl EventRegistry {
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register an event listener for a pattern
    /// Pattern examples: "orders:*", "orders:123", "users:*"
    pub async fn register<L>(&self, pattern: impl Into<String>, listener: L)
    where
        L: EventListener + 'static,
    {
        let mut listeners = self.listeners.write().await;
        listeners
            .entry(pattern.into())
            .or_insert_with(Vec::new)
            .push(Arc::new(listener));
    }

    /// Register a listener that's already wrapped in Arc
    pub async fn register_arc(&self, pattern: impl Into<String>, listener: Arc<dyn EventListener>) {
        let mut listeners = self.listeners.write().await;
        listeners
            .entry(pattern.into())
            .or_insert_with(Vec::new)
            .push(listener);
    }

    /// Emit an event to matching listeners
    pub async fn emit(&self, event: Event) -> Result<()> {
        let listeners = self.listeners.read().await;
        let pattern = event.pattern();

        // Find matching patterns
        let mut matched_listeners = Vec::new();

        // Exact match
        if let Some(exact) = listeners.get(&pattern) {
            matched_listeners.extend(exact.iter().cloned());
        }

        // Wildcard match (table:*)
        let wildcard_pattern = format!("{}:*", event.table);
        if let Some(wildcard) = listeners.get(&wildcard_pattern) {
            matched_listeners.extend(wildcard.iter().cloned());
        }

        // Global wildcard (*)
        if let Some(global) = listeners.get("*") {
            matched_listeners.extend(global.iter().cloned());
        }

        // Notify all matched listeners
        for listener in matched_listeners {
            // Clone event for each listener
            listener.on_event(event.clone()).await?;
        }

        Ok(())
    }

    /// List all registered patterns
    pub async fn patterns(&self) -> Vec<String> {
        let listeners = self.listeners.read().await;
        listeners.keys().cloned().collect()
    }
}

impl Default for EventRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Add chrono dependency for timestamps
