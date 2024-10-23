use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub ip_address: String,
    pub location: String,
    pub updated_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
}