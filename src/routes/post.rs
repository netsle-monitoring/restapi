use crate::crypto;
use crate::database;
use crate::guards::{RefreshApiKey, Admin};
use crate::guards::{self, auth};
use crate::MainDbConn;
use rocket::request::Form;
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

/**
 * This endpoint requires a form data which contains two fields
 * username: String
 * password: String
 * If the credentials are correct against the database the client should get
 * a 200 Response code and a body which contains:
 * expires: usize | this number represents the timestamp at which the access token is going to expire
 * access_token: String | Self explanatory
 * refresh_token: String | A token which is required to use in order to persist sessions and API communication overall
 */
#[post("/login", data = "<login>")]
pub fn login(
    conn: MainDbConn,
    login: Form<guards::LoginCredentials>,
) -> Result<content::Json<String>, BadRequest<content::Json<String>>> {
    let invalid_creds_response = ErrorResponse {
        message: "Invalid Credentials!",
    };

    let user_result = database::users::get_user(&*conn, &login.username);

    // TODO: Find a way of not cloning this piece of code.
    if user_result.is_none() {
        return Err(BadRequest(Some(content::Json(
            serde_json::to_string(&invalid_creds_response).unwrap(),
        ))));
    }

    let user = user_result.unwrap();
    let password_validity =
        crypto::verify_password(&login.username, user.hashed_pw, &login.password);

    if password_validity.is_err() {
        return Err(BadRequest(Some(content::Json(
            serde_json::to_string(&invalid_creds_response).unwrap(),
        ))));
    }

    let (access_token, expiry, refresh_token) =
        auth::generate_tokens(String::from(&login.username), user.is_admin);

    let response = SuccessfulLoginResponse {
        refresh_token,
        access_token,
        expiry,
    };

    database::users::update_refresh_token(&*conn, &login.username, &response.refresh_token);
    Ok(content::Json(serde_json::to_string(&response).unwrap()))
}

/**
 * The way this endpoint works is that
 * whenever you want to get a new refresh token you would
 * want to send an EMPTY request with an header by the name of
 * X-Refresh-Token
 * If the refresh token is correct the response will be the same as
 * you were to login with a username / password
 */
#[post("/refresh_token")]
pub fn refresh_token(
    conn: MainDbConn,
    refresh: RefreshApiKey,
) -> Result<content::Json<String>, BadRequest<content::Json<String>>> {
    let user_result = database::users::get_user(&*conn, &refresh.0.to_owned());

    // TODO: Find a way of not cloning this piece of code.
    if user_result.is_none() {
        return Err(BadRequest(Some(content::Json(
            serde_json::to_string("todo").unwrap(),
        ))));
    }

    let user = user_result.unwrap();

    let (access_token, expiry, refresh_token) = auth::generate_tokens(String::from(&refresh.0), user.is_admin);

    let response = SuccessfulLoginResponse {
        refresh_token,
        access_token,
        expiry,
    };

    database::users::update_refresh_token(&*conn, &refresh.0, &response.refresh_token);
    Ok(content::Json(serde_json::to_string(&response).unwrap()))
}

#[post("/create_user", data = "<user_creation_details>")]
pub fn create_user(
    conn: MainDbConn,
    user_creation_details: Form<guards::UserCreationCredentials>,
    admin: Admin,
) -> Result<content::Json<String>, BadRequest<content::Json<String>>> {
    let user_result = database::users::create_user(
        &*conn,
        String::from(&user_creation_details.username),
        String::from(&user_creation_details.password),
        user_creation_details.admin,
    );

    match user_result {
        Err(e) => {
            return Err(BadRequest(Some(content::Json(
                serde_json::to_string(&ErrorResponse {
                    message: e,
                })
                .unwrap(),
            ))));
        }
        _ => {}
    }
    Ok(content::Json("{}".to_owned()))
}

#[options("/refresh_token")]
pub fn refresh_token_options() -> &'static str {
    ""
}