// mongo_repo.rs
use std::env;
extern crate dotenv;
use dotenv::dotenv;
use mongodb::{Client, Database};
use log::{error, info};

pub struct MongoRepo {
    pub db: Database,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok(); // Load environment variables from `.env`

        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => {
                error!("Error loading MONGOURI from environment variables");
                String::from("mongodb://localhost:27017") // Fallback URI
            }
        };

        let client = Client::with_uri_str(&uri)
            .await
            .expect("Failed to initialize MongoDB client");
        
        let db = client.database("best-fit-jobs"); 

        info!("Connected to mongodb!");

        MongoRepo { db } 
    }
}
