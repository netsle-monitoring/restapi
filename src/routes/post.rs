use crate::crypto;
use crate::database;
use crate::guards::RefreshApiKey;
use crate::guards::{self, auth};
use crate::MainDbConn;
use rocket::request::{Form, State};
use rocket::response::content;
use rocket::response::status::BadRequest;
use serde::Serialize;

#[derive(Serialize)]
struct SuccessfulLoginResponse {
    access_token: String,
    refresh_token: String,
    expiry: usize,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: &'static str,
}

#[post("/login", data = "<login>")]
pub fn login(
    conn: MainDbConn,
    login: Form<guards::LoginCredentials>,
) -> Result<content::Json<String>, BadRequest<content::Json<String>>> {
    let invalidCredsResponse = ErrorResponse {
        message: "Invalid Credentials!",
    };

    let user_result = database::users::get_user(&*conn, &login.username);

    // TODO: Find a way of not cloning this piece of code.
    if user_result.is_none() {
        return Err(BadRequest(Some(content::Json(
            serde_json::to_string(&invalidCredsResponse).unwrap(),
        ))));
    }

    let user = user_result.unwrap();
    let password_validity =
        crypto::verify_password(&login.username, user.hashed_pw, &login.password);

    if password_validity.is_err() {
        return Err(BadRequest(Some(content::Json(
            serde_json::to_string(&invalidCredsResponse).unwrap(),
        ))));
    }

    let (access_token, expiry, refresh_token) =
        auth::generate_tokens(String::from(&login.username));

    let response = SuccessfulLoginResponse {
        refresh_token,
        access_token,
        expiry,
    };

    database::users::update_refresh_token(&*conn, &login.username, &response.refresh_token);
    Ok(content::Json(serde_json::to_string(&response).unwrap()))
}

#[post("/refresh")]
pub fn refresh(
    conn: MainDbConn,
    refresh: RefreshApiKey,
) -> Result<content::Json<String>, BadRequest<content::Json<String>>> {
    let (access_token, expiry, refresh_token) = auth::generate_tokens(String::from(&refresh.0));

    let response = SuccessfulLoginResponse {
        refresh_token,
        access_token,
        expiry,
    };

    database::users::update_refresh_token(&*conn, &refresh.0, &response.refresh_token);
    Ok(content::Json(serde_json::to_string(&response).unwrap()))
}
