pub mod cpu;
pub mod memory;
pub mod network;

pub use cpu::collect_cpu_usage;
pub use memory::collect_memory_usage;
pub use network::capture_packets;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use mongodb::{bson::doc, Collection}; // Import the `doc` macro from `mongodb::bson`

#[derive(Serialize, Deserialize)] 
pub struct Metric {
    pub timestamp: i64,
    pub cpu_usage: f64,
    pub memory_usage: f64,
}
pub async fn collect_metrics(collection: Arc<Mutex<Collection<Metric>>>) -> Result<(), Box<dyn std::error::Error>> {
    let cpu_usage = collect_cpu_usage().await;
    let memory_usage = collect_memory_usage().await;

    let metric = Metric {
        timestamp: Utc::now().timestamp(),
        cpu_usage,
        memory_usage,
    };

    let mut collection = collection.lock().await;
    collection.insert_one(metric).await?;

    // Remove old metrics (older than 30 minutes)
    let cutoff_time = (Utc::now() - chrono::Duration::minutes(30)).timestamp();
    collection.delete_many(doc! { "timestamp": { "$lt": cutoff_time } }).await?;

    Ok(())
}
