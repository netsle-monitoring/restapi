use crate::guards::{self, auth};
use rocket::request::{Form, State};
use rocket::response::status::BadRequest;
use rocket::response::content;
use serde::{Serialize};
// use crate::database::users;

#[derive(Serialize)]
struct SuccessfulLoginResponse {
    access_token: String,
    refresh_token: String,
    expiry: usize
}

#[derive(Serialize)]
struct ErrorResponse {
    message: &'static str
}

#[post("/login", data = "<login>")]
pub fn login(
    users: State<guards::Users>,
    login: Form<guards::LoginCredentials>,
) -> Result<content::Json<String>, BadRequest<content::Json<String>>> {
    let static_username = &users.0[0].0;
    let static_password = &users.0[0].1;

    if static_password != &login.password || static_username != &login.username {
        let response = ErrorResponse {
            message: "Invalid Credentials!"
        };

        return Err(
            BadRequest(
                Some(content::Json(
                    serde_json::to_string(&response).unwrap()
                ))
            )
        )
    }

    let (access_token, expiry, refresh_token) = auth::generate_tokens(&login);

    let response = SuccessfulLoginResponse {
        refresh_token,
        access_token,
        expiry
    };

    Ok(
        content::Json(serde_json::to_string(&response).unwrap())
    )
}
