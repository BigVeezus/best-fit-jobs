use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject, typically the user ID or email
    pub exp: usize,  // Expiration time as UNIX timestamp
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// Function to generate JWT token for a given user
pub fn generate_jwt(user_email: &str) -> String {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let expiration = Utc::now()
        .checked_add_signed(Duration::days(2)) // Token valid for 1 hour
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_email.to_owned(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .expect("JWT encoding should work")
}

// // Function to validate a JWT token and return claims if valid
// pub fn validate_jwt(token: &str) -> Option<Claims> {
//     let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

//     let token_data = decode::<Claims>(
//         token,
//         &DecodingKey::from_secret(secret.as_ref()),
//         &Validation::default(),
//     );

//     token_data.map(|data| data.claims).ok()
// }
