use chrono::{Duration as ChronoDuration, Utc};
use futures::TryStreamExt;
use mongodb::{bson::doc, Client, Collection, Database};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use sysinfo::{CpuExt, System, SystemExt};
use tokio::runtime::Runtime;
use tokio::sync::Mutex as TokioMutex;
use pnet::packet::ethernet::EthernetPacket;

mod metrics {
    pub mod network;
}
use metrics::network::capture_packets;

#[derive(Serialize, Deserialize)]
struct Metric {
    timestamp: i64,
    cpu_usage: f64,
    memory_usage: f64,
}

#[tokio::main]
async fn main() {
    // Initialize MongoDB client
    let mongo_client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    let db: Database = mongo_client.database("metrics_db");
    let collection: Collection<Metric> = db.collection("metrics");

    // Clone the collection for use in the thread
    let collection = Arc::new(TokioMutex::new(collection));

    // Spawn thread for data collection and MongoDB operations
    let collection_clone = collection.clone();
    thread::spawn(move || {
        let rt = Runtime::new().unwrap();

        loop {
            rt.block_on(async {
                // Create a system object
                let mut system = System::new();
                system.refresh_all();

                // Collect metrics
                let cpu_usage_percentage = system.global_cpu_info().cpu_usage() as f64;
                let memory_usage_percentage = system.used_memory() as f64 / system.total_memory() as f64 * 100.0;
                let metric = Metric {
                    timestamp: Utc::now().timestamp(),
                    cpu_usage: cpu_usage_percentage,
                    memory_usage: memory_usage_percentage,
                };

                // Insert the metric into MongoDB
                let mut collection = collection_clone.lock().await;
                if let Err(e) = collection.insert_one(metric).await {
                    eprintln!("Failed to insert metric: {:?}", e);
                }

                // Remove old metrics (older than 30 minutes)
                let cutoff_time = (Utc::now() - ChronoDuration::minutes(30)).timestamp();
                if let Err(e) = collection.delete_many(
                    doc! { "timestamp": { "$lt": cutoff_time } }
                ).await {
                    eprintln!("Failed to delete old metrics: {:?}", e);
                }

                // Sleep before collecting the next set of metrics
                thread::sleep(Duration::from_secs(1));
            });
        }
    });

    // Define packet handler
    let packet_handler: Arc<Mutex<dyn Fn(EthernetPacket) + Send + Sync>> = Arc::new(Mutex::new(|packet: EthernetPacket| {
        println!("Received packet: {:?}", packet);
    }));

    // Start capturing network packets
    thread::spawn(move || {
        capture_packets(packet_handler);
    });

    // Keep the main thread alive
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}
