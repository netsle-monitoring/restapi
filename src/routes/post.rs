use crate::guards::{self, auth};
use rocket::http::Status;
use rocket::request::{Form, State};
use rocket::response::status;

#[post("/login", data = "<login>")]
pub fn login(
    users: State<guards::Users>,
    login: Form<guards::LoginCredentials>,
) -> status::Custom<String> {
    let static_username = &users.0[0].0;
    let static_password = &users.0[0].1;

    if static_password != &login.password || static_username != &login.username {
        return status::Custom(
            Status::new(401, "Invalid credentials"),
            String::from("Invalid credentials"),
        );
    }

    let token = auth::generate_jwt(&login);
    status::Custom(Status::new(200, "Success"), token)
}
