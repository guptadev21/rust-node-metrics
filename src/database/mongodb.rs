use mongodb::{Client, Database, Collection};
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use crate::metrics::Metric;

pub async fn initialize_mongodb() -> (Arc<Mutex<Collection<Metric>>>, Runtime) {
    let mongo_client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    let db: Database = mongo_client.database("metrics_db");
    let collection: Collection<Metric> = db.collection("metrics");
    let collection = Arc::new(Mutex::new(collection));
    let rt = Runtime::new().unwrap();
    (collection, rt)
}
