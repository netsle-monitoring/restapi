use jsonwebtoken::TokenData;
use serde::{Deserialize, Serialize};

// (Username, Password)
pub struct Users(pub Vec<(String, String)>);

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    pub exp: usize,
    sub: String, // username
    iss: String, // Netsle
}

pub struct ApiKey(pub TokenData<JWTClaims>);
pub struct RefreshApiKey(pub String); // This string is for the username so a new token can be generated

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
