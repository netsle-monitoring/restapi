use jsonwebtoken::TokenData;
use serde::{Deserialize, Serialize};

// (Username, Password)
pub struct Users(pub Vec<(String, String)>);

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    exp: usize,
    sub: String, // username
    iss: String, // Netsle
}

pub struct ApiKey(pub TokenData<JWTClaims>);

#[derive(Debug)]
pub enum ApiKeyError {
    Invalid,
    Expired,
    Missing,
}

#[derive(Debug, Clone)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

pub mod auth;
