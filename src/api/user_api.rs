use crate::{models::user_model::User, repository::user_repo::UserRepo};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

#[post("/user")]
pub async fn create_user(db: Data<UserRepo>, new_user: Json<User>) -> HttpResponse {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        location: new_user.location.to_owned(),
        password: new_user.password.to_owned(),
        ip_address: new_user.ip_address.to_owned(),
        updated_at: None,
        created_at: None,
    };
    let user_detail = db.create_user(data).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}