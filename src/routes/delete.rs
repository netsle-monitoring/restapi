use serde::Serialize;
use rocket::request::Form;
use rocket::response::content;
use rocket::response::status::BadRequest;
use crate::database;
use crate::guards::{UserDeleteForm, ApiKey, Admin};
use crate::MainDbConn;

#[derive(Serialize)]
struct ErrorResponse {
    message: &'static str,
}

#[delete("/admin/delete_user", data = "<delete_form>")]
pub fn delete_user(
    key: ApiKey,
    _admin: Admin,
    conn: MainDbConn,
    delete_form: Form<UserDeleteForm>
) -> Result<content::Json<String>, BadRequest<&'static str>> {
    if key.0.claims.sub == delete_form.username || delete_form.username == "netsle" {
        return Err(BadRequest(Some("{\"message\": \"Can't delete yourself nor netsle you silly goose!\"}")));
    }

    let result = database::users::delete_user(&*conn, &delete_form.username);

    match result {
        Ok(_) => {
            Ok(content::Json("{}".to_string()))
        },
        Err(_) => {
            Err(BadRequest(Some("{\"message\": \"Something bad happened with our database :)\"}")))
        }
    }
}