use crate::elastic;
use crate::guards::ApiKey;
use rocket::response::content;
use rocket::State;
use serde_json::{Value};

#[get("/")]
pub fn index() -> &'static str {
    "Hello, netsle!"
}

#[get("/netstats")]
pub fn network_stats(
    _access: ApiKey,
    elastic: State<elastic::ElasticClient>,
) -> content::Json<String> {
    let response = elastic.0.get_packet_count_since("netsle");

    // v[""]
    content::Json(response)
}
