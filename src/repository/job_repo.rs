use crate::models::job_model::Job;
use mongodb::{
    bson::{extjson::de::Error, DateTime},
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
}
