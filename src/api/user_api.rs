use crate::config::auth::{generate_jwt, LoginRequest};
use crate::{models::user_model::User, repository::user_repo::UserRepo};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use actix_web::{web, Responder};
use bcrypt::verify;
use log::{error, info};
use serde_json::json;

#[post("/user")]
pub async fn create_user(db: Data<UserRepo>, new_user: Json<User>) -> HttpResponse {
    let email = new_user.email.to_owned();

    match db.find_by_email(&email).await {
        Ok(Some(_)) => {
            info!("User with email {} already exists", email);
            return HttpResponse::BadRequest().json("User already exists");
        }

        Ok(None) => {
            // No user found with the given email, proceed with creating a new user
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
                Ok(user) => {
                    info!("User created successfully!");
                    let token = generate_jwt(&email);

                    let response_body = json!({
                        "userId": user.inserted_id.to_string(), // Convert ObjectId to String
                        "token": token,
                    });

                    return HttpResponse::Created().json(response_body);
                }
                Err(err) => {
                    error!("{}", err);
                    return HttpResponse::InternalServerError().body(err.to_string());
                }
            }
        }

        Err(err) => {
            // Error while querying the database
            error!("Error querying user by email: {}", err);
            return HttpResponse::InternalServerError().body("Internal server error");
        }
    }
}

#[post("/user/login")]
async fn login(db: Data<UserRepo>, login_data: web::Json<LoginRequest>) -> impl Responder {
    let user_email = login_data.email.clone();
    let user_password = login_data.password.clone();

    // Query the user by email from MongoDB
    match db.find_by_email(&user_email).await {
        Ok(Some(user)) => {
            // Compare the provided password with the stored hashed password
            let is_valid = verify(&user_password, &user.password).unwrap_or(false);

            if is_valid {
                // Password is valid, generate JWT
                let token = generate_jwt(&user_email);
                info!("User {} logged in successfully!", user_email);

                let response_body = json!({
                    "userId": user.id.unwrap().to_string(), // Convert ObjectId to String
                    "token": token,
                });

                HttpResponse::Ok().json(response_body)
            } else {
                // Invalid password
                info!("Invalid password attempt for user: {}", user_email);
                HttpResponse::Unauthorized().json("Invalid credentials")
            }
        }
        Ok(None) => {
            // No user found with the provided email
            info!("User with email {} not found", user_email);
            HttpResponse::NotFound().json("User not found")
        }
        Err(err) => {
            // Error while querying the database
            error!("Error querying the user by email: {}", err);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
