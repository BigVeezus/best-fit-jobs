use mongodb::{
    bson::extjson::de::Error,
    results::InsertOneResult, Collection,
};
use crate::models::user_model::User;

pub struct UserRepo {
    col: Collection<User>,
}

impl UserRepo {
    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            email: new_user.email,
            password: new_user.password,
            ip_address: new_user.ip_address,
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
