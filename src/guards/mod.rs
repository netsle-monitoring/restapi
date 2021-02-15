use jsonwebtoken::TokenData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    pub exp: usize,
    pub sub: String, // username
    iss: String, // Netsle
}

pub struct ApiKey(pub TokenData<JWTClaims>);
pub struct RefreshApiKey(pub String); // This string is for the username so a new token can be generated
pub struct Admin(pub String); // This string is for the username

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

#[derive(Debug, Clone)]
pub struct UserCreationCredentials {
    pub username: String,
    pub password: String,
    pub admin: bool
}

#[derive(Debug, Clone)]
pub struct BlacklistEntryCreation {
    pub ip: String
}

#[derive(Debug, Clone)]
pub struct UserDeleteForm {
    pub username: String
 }

pub mod auth;
