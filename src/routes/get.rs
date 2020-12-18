use crate::elastic;
use rocket::State;
use rocket::{
    Outcome,
    http::{Status},
    response::{content},
    request::{self, Request, FromRequest}
};

#[get("/")]
pub fn index() -> &'static str {
    "Hello, netsle!"
}

#[get("/netstats")]
pub fn network_stats(elastic: State<elastic::ElasticClient>) -> content::Json<String> {
    let response = elastic.0.match_all_for_index("netsle");
    content::Json(response)
}

pub struct ApiKey(String);

#[derive(Debug)]
pub enum ApiKeyError {
    Invalid
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;
    fn from_request(_request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid))
    }
}

#[get("/home")]
pub fn home(_key: ApiKey) -> &'static str {
    "Sensitive data."
}