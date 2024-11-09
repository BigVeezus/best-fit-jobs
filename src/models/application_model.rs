use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Application {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub job_id: ObjectId,
    pub resume: Option<String>,
    pub full_name: String,
    pub email: String,
    pub cover_letter: Option<String>,
    pub address: String,
    pub country: String,
    pub score: Option<f32>,
    pub updated_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
}
