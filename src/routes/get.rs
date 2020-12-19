use crate::elastic;
use crate::guards::ApiKey;
use rocket::response::content;
use rocket::State;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, netsle!"
}

#[get("/netstats")]
pub fn network_stats(
    _access: ApiKey,
    elastic: State<elastic::ElasticClient>,
) -> content::Json<String> {
    let response = elastic.0.match_all_for_index("netsle");
    content::Json(response)
}
