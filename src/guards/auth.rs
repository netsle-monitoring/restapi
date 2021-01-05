use super::{ApiKey, ApiKeyError, JWTClaims, LoginCredentials, RefreshApiKey};
use crate::database;
use crate::MainDbConn;
use jsonwebtoken::errors::ErrorKind::{ExpiredSignature, InvalidSignature, InvalidToken};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::{
    http::Status,
    request::{self, FormItems, FromForm, FromRequest, Request},
    Outcome,
};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let header_map = request.headers();

        if !header_map.contains("Autherization") {
            return Outcome::Failure((Status::BadRequest, ApiKeyError::Missing));
        }

        let auth_header = header_map.get("Autherization").next().unwrap();

        if !(auth_header.len() > 7) {
            return Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid));
        }

        let token = &auth_header[7..]; // Remove the "Bearer " part!

        let validation = Validation {
            iss: Some("Netsle".to_string()),
            ..Validation::default()
        };

        let jwt_secret = env::var("JWT_SECRET").unwrap();

        // Obviously this won't be the production secret, just for now
        let token_data = match decode::<JWTClaims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &validation,
        ) {
            Ok(data) => data,
            Err(err) => match *err.kind() {
                ExpiredSignature => {
                    return Outcome::Failure((Status::Forbidden, ApiKeyError::Expired))
                }
                InvalidToken | InvalidSignature => {
                    return Outcome::Failure((Status::Forbidden, ApiKeyError::Invalid))
                }
                _ => panic!(format!("{:?}", *err.kind())),
            },
        };

        Outcome::Success(ApiKey(token_data))
    }
}

impl<'f> FromForm<'f> for LoginCredentials {
    type Error = ();

    fn from_form(credentials: &mut FormItems<'f>, strict: bool) -> Result<LoginCredentials, ()> {
        let mut username = None;
        let mut password = None;

        for credential in credentials {
            match credential.key.as_str() {
                "username" if username.is_none() => {
                    let decoded = credential.value.url_decode().map_err(|_| ())?;
                    username = Some(decoded)
                }
                "password" if password.is_none() => {
                    let decoded = credential.value.url_decode().map_err(|_| ())?;
                    password = Some(decoded)
                }
                _ if strict => return Err(()),
                _ => {}
            }
        }

        Ok(LoginCredentials {
            username: username.unwrap(),
            password: password.unwrap(),
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for RefreshApiKey {
    type Error = ApiKeyError;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let header_map = request.headers();
        let conn = request.guard::<MainDbConn>().unwrap();

        if !header_map.contains("X-Refresh-Token") {
            return Outcome::Failure((Status::BadRequest, ApiKeyError::Missing));
        }

        let auth_header = header_map.get("X-Refresh-Token").next().unwrap();

        if (auth_header.len() == 0) {
            println!("Right here boy");
            return Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid));
        }

        let token = &auth_header; // Remove the "Bearer " part!

        let username_of_refresh = database::users::get_username_for_refresh_token(&*conn, token);

        // if Some means exists
        if username_of_refresh.is_none() {
            return Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid));
        }

        let username_of_refresh = username_of_refresh.unwrap();

        let validation = Validation {
            iss: Some("Netsle".to_string()),
            ..Validation::default()
        };

        let jwt_secret = env::var("JWT_REFRESH_SECRET").unwrap();

        // Obviously this won't be the production secret, just for now
        let token_data = match decode::<JWTClaims>(
            token,
            &DecodingKey::from_secret(format!("{}{}", &jwt_secret, &username_of_refresh).as_ref()),
            &validation,
        ) {
            Ok(data) => data,
            Err(err) => match *err.kind() {
                ExpiredSignature => {
                    return Outcome::Failure((Status::Forbidden, ApiKeyError::Expired))
                }
                InvalidToken | InvalidSignature => {
                    return Outcome::Failure((Status::Forbidden, ApiKeyError::Invalid))
                }
                _ => panic!(format!("{:?}", *err.kind())),
            },
        };

        // can't fail since we check if it's a none beforehand.
        Outcome::Success(RefreshApiKey(username_of_refresh))
    }
}

// (access, expiry, refresh)
pub fn generate_tokens(username: String) -> (String, usize, String) {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let jwt_secret = env::var("JWT_SECRET").unwrap();
    let jwt_refresh_secret = env::var("JWT_REFRESH_SECRET").unwrap();
    let regular_jwt_expiry = env::var("TOKEN_EXPIRY_IN_MINUTES")
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let refresh_jwt_expiry = env::var("REFRESH_TOKEN_EXPIRY_IN_MINUTES")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let access_token_claims = JWTClaims {
        iss: String::from("Netsle"),
        sub: String::from(&username),
        exp: (since_the_epoch + (regular_jwt_expiry * 60)) as usize, // Expires in whatever minutes are inside .env
    };

    // TODO: Have the refresh secret unique between users
    let refresh_token_claims = JWTClaims {
        iss: String::from("Netsle"),
        sub: String::from(&username),
        exp: (since_the_epoch + (refresh_jwt_expiry * 60)) as usize, // Expires in whatever minutes are inside .env
    };

    let access_token = encode(
        &Header::default(),
        &access_token_claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    let refresh_token = encode(
        &Header::default(),
        &refresh_token_claims,
        &EncodingKey::from_secret(
            format!("{}{}", jwt_refresh_secret, String::from(&username)).as_ref(),
        ),
    )
    .unwrap();

    return (access_token, access_token_claims.exp, refresh_token);
}
