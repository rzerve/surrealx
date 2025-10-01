//! Basic SurrealX example demonstrating custom functions and events

use surrealx::{SurrealX, Module, ServerConfig};
use serde_json::{json, Value};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Custom function: calculate tax
    async fn calculate_tax(args: Vec<Value>) -> Result<Value> {
        let price = args.get(0)
            .and_then(|v| v.as_f64())
            .ok_or_else(|| anyhow::anyhow!("Invalid price"))?;

        let tax_rate = args.get(1)
            .and_then(|v| v.as_f64())
            .unwrap_or(0.15);

        Ok(json!(price * tax_rate))
    }

    // Custom function: format currency
    async fn format_currency(args: Vec<Value>) -> Result<Value> {
        let amount = args.get(0)
            .and_then(|v| v.as_f64())
            .ok_or_else(|| anyhow::anyhow!("Invalid amount"))?;

        Ok(json!(format!("${:.2}", amount)))
    }

    // Create a business module
    let business = Module::new("business")
        .with_function("calculate_tax", calculate_tax)
        .with_function("format_currency", format_currency)
        .with_listener("orders:*", |event| async move {
            println!("📦 Order event: {:?}", event);
            Ok(())
        })
        .with_listener("payments:*", |event| async move {
            println!("💰 Payment event: {:?}", event);
            Ok(())
        });

    // Start SurrealX with extensions
    println!("🚀 Starting SurrealX with business extensions...\n");

    SurrealX::new()
        .with_module(business)
        .serve(ServerConfig::default())
        .await?;

    println!("\n📚 SQL Usage Examples:");
    println!("  SELECT ext::calculate_tax(100.0, 0.15) AS tax;");
    println!("  SELECT ext::format_currency(1234.56) AS formatted;");
    println!("  SELECT sx::emit('orders:created', {{ order_id: 123 }});");

    Ok(())
}
