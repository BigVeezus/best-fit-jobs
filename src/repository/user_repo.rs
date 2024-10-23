use bcrypt::{hash, DEFAULT_COST};
use mongodb::{
    bson::{extjson::de::Error, DateTime},
    results::InsertOneResult, Collection, Database
};
use crate::models::user_model::User;

pub struct UserRepo {
    col: Collection<User>,
}

impl UserRepo {
    pub fn new(db: &Database) -> Self {
        let col: Collection<User> = db.collection("user"); // Create the User collection
        UserRepo { col }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {

        let hashed_password = hash(new_user.password, DEFAULT_COST)
            .expect("Error hashing password");

            let now = DateTime::now();

        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            email: new_user.email,
            password: hashed_password,
            ip_address: new_user.ip_address,
            updated_at: Some(now),
            created_at: Some(now)
        };
        let user = self
            .col
            .insert_one(new_doc)
            .await
            .ok()
            .expect("Error creating user");
        Ok(user)
    }
}
