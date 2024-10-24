use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum JobCategory {
    IT,
    Healthcare,
    Education,
    Engineering,
    Finance,
    Marketing,
    Sales,
    Legal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum JobType {
    FullTime,
    Contract,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum JobAvailability {
    Remote,
    Hybrid,
    Freelance,
    Office,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Job {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub job_title: String,
    pub category: JobCategory,
    pub tags: Vec<ObjectId>,
    pub job_type: JobType,
    pub job_availability: JobAvailability,
    pub job_duration: u32,
    pub contract_duration: u32,
    pub location: String,
    pub updated_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
}

impl FromStr for JobAvailability {
    type Err = &'static str; // You can define a more descriptive error type if needed

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "remote" => Ok(JobAvailability::Remote),
            "hybrid" => Ok(JobAvailability::Hybrid),
            "freelance" => Ok(JobAvailability::Freelance),
            "office" => Ok(JobAvailability::Office),
            _ => Err("Invalid job availability"),
        }
    }
}
