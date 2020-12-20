use super::{ApiKey, ApiKeyError, JWTClaims, LoginCredentials};
use jsonwebtoken::errors::ErrorKind::ExpiredSignature;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::{
    http::Status,
    request::{self, FormItems, FromForm, FromRequest, Request},
    Outcome,
};
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

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
        // Obviously this won't be the production secret, just for now
        let token_data = match decode::<JWTClaims>(
            token,
            &DecodingKey::from_secret("ef2d6ea9-a99a-4158-981a-7fa890ca22f7".as_ref()),
            &validation,
        ) {
            Ok(data) => data,
            Err(err) => match *err.kind() {
                ExpiredSignature => {
                    return Outcome::Failure((Status::Forbidden, ApiKeyError::Expired))
                }
                _ => panic!(),
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

pub fn generate_jwt(credentials: &LoginCredentials) -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    
    let regular_jwt_expiry = env::var("TOKEN_EXPIRY_IN_MINUTES").unwrap().parse::<u64>().unwrap();
    let jwt_secret = env::var("JWT_SECRET").unwrap();

    let claims = JWTClaims {
        iss: String::from("Netsle"),
        sub: credentials.clone().username,
        exp: (since_the_epoch + (regular_jwt_expiry * 60)) as usize, // Expires in whatever minutes are inside .env
    };

    // Obviously this won't be the production secret, just for now
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();
    return token;
}
