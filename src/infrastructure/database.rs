use std::env;
use mongodb::{Client, Database};

pub async fn init() -> Database {
    let uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let db_name = env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME must be set");
    let client = Client::with_uri_str(&uri).await.expect("Failed to connect to MongoDB");
    client.database(&db_name)
}
