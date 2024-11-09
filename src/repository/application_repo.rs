use crate::models::application_model::Application;
use futures::TryStreamExt;
use mongodb::{
    bson::{extjson::de::Error, DateTime, Document},
    results::InsertOneResult,
    Collection, Database,
};

pub struct ApplicationRepo {
    col: Collection<Application>,
}

impl ApplicationRepo {
    pub fn new(db: &Database) -> Self {
        let col: Collection<Application> = db.collection("applications"); // Create the application collection
        ApplicationRepo { col }
    }

    pub async fn create_application(&self, new_app: Application) -> Result<InsertOneResult, Error> {
        let now = DateTime::now();

        let new_doc = Application {
            id: None,
            job_id: new_app.job_id,
            full_name: new_app.full_name,
            resume: new_app.resume,
            email: new_app.email,
            cover_letter: new_app.cover_letter,
            address: new_app.address,
            country: new_app.country,
            score: new_app.score,
            updated_at: Some(now),
            created_at: Some(now),
        };
        let application = self
            .col
            .insert_one(new_doc)
            .await
            .ok()
            .expect("Error creating application");
        Ok(application)
    }

    pub async fn get_all_applications(&self) -> Result<Vec<Application>, mongodb::error::Error> {
        let filter = Document::new();

        let cursor = self.col.find(filter).await?;

        // Convert the cursor into a vector of Application structs
        let application: Vec<Application> = cursor.try_collect().await?;

        Ok(application)
    }
}
