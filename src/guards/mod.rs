use serde::{Serialize, Deserialize};

// (Username, Password)
pub struct Users(pub Vec<(String, String)>);

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    exp: usize,
    sub: String, // username
    iss: String // Netsle
}