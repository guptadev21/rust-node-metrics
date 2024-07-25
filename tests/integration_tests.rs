use mongodb::{bson::{doc, Bson}, Collection, Client};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use tokio;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metric {
    pub timestamp: i64,
    pub cpu_usage: f64,
    pub memory_usage: f64,
}

#[tokio::test]
async fn test_insert_and_delete_metrics() {
    // Initialize MongoDB client and collection
    let mongo_client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    let db = mongo_client.database("metrics_db");
    let collection: Collection<Metric> = db.collection("metrics");

    // Clear collection before test
    collection.delete_many(doc! {}).await.unwrap();

    // Insert a metric
    let metric = Metric {
        timestamp: Utc::now().timestamp(),
        cpu_usage: 50.0,
        memory_usage: 60.0,
    };

    // Insert the metric
    let insert_result = collection.insert_one(metric.clone()).await.unwrap();

    // Verify that the metric was inserted
    let inserted_id = insert_result.inserted_id.as_object_id().unwrap();
    let count = collection.count_documents(doc! { "_id": inserted_id }).await.unwrap();
    println!("Document count after insertion: {}", count);
    assert_eq!(count, 1);

    // Simulate passing time or adjust for actual time
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Perform deletion using the inserted _id
    let delete_result = collection.delete_one(doc! { "_id": inserted_id }).await.unwrap();
    println!("Deleted count: {}", delete_result.deleted_count);

    // Verify that the metric was deleted
    let count_after_deletion = collection.count_documents(doc! { "_id": inserted_id }).await.unwrap();
    println!("Document count after deletion: {}", count_after_deletion);
    assert_eq!(count_after_deletion, 0);
}
