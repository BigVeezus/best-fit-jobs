use crate::models::job_model::Job;
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::error::Error;
use mongodb::{
    bson::{DateTime, Document},
    results::InsertOneResult,
    Collection, Database,
};

pub struct JobRepo {
    col: Collection<Job>,
}

impl JobRepo {
    pub fn new(db: &Database) -> Self {
        let col: Collection<Job> = db.collection("jobs"); // Create the User collection
        JobRepo { col }
    }

    pub async fn create_job(&self, new_job: Job) -> Result<InsertOneResult, Error> {
        let now = DateTime::now();

        let new_doc = Job {
            id: None,
            job_title: new_job.job_title,
            job_image: new_job.job_image,
            job_description: new_job.job_description,
            location: new_job.location,
            category: new_job.category,
            tags: new_job.tags,
            job_type: new_job.job_type,
            job_availability: new_job.job_availability,
            job_duration: new_job.job_duration,
            contract_duration: new_job.contract_duration,
            updated_at: Some(now),
            created_at: Some(now),
        };
        let job = self
            .col
            .insert_one(new_doc)
            .await
            .ok()
            .expect("Error creating job");
        Ok(job)
    }

    pub async fn get_all_jobs(&self) -> Result<Vec<Job>, mongodb::error::Error> {
        let filter = Document::new();

        let cursor = self.col.find(filter).await?;

        // Convert the cursor into a vector of Job structs
        let jobs: Vec<Job> = cursor.try_collect().await?;

        Ok(jobs)
    }

    pub async fn fetch_job_tags(
        &self,
        job_id: &ObjectId,
    ) -> Result<Option<Vec<String>>, mongodb::error::Error> {
        let filter = doc! { "_id": job_id };

        match self.col.find_one(filter).await {
            Ok(Some(job)) => {
                // If the job is found, return the tags
                Ok(Some(job.tags.unwrap_or_else(|| vec![]))) // Return empty vec if no tags
            }
            Ok(None) => {
                // If no job is found with the given job_id, return None
                Ok(None)
            }
            Err(e) => {
                // Any other error from MongoDB is returned as-is
                Err(e)
            }
        }
    }
}
