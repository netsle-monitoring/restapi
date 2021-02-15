use serde::Serialize;
use rocket::request::Form;
use rocket::response::content;
use rocket::http::Status;
use crate::database;
use crate::guards::{UserDeleteForm, ApiKey, Admin};
use crate::MainDbConn;

#[derive(Serialize)]
struct ErrorResponse {
    message: &'static str,
}

#[delete("/admin/delete_user", data = "<delete_form>")]
pub fn delete_user(
    _key: ApiKey,
    _admin: Admin,
    conn: MainDbConn,
    delete_form: Form<UserDeleteForm>
) -> Result<content::Json<String>, Status> {
    let result = database::users::delete_user(&*conn, &delete_form.username);

    match result {
        Ok(_) => {
            Ok(content::Json("{}".to_string()))
        },
        Err(_) => {
            Err(Status::new(500, "Something bad happened with our database :)"))
        }
    }
}