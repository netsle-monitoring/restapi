#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use reqwest;
use rocket::State;
use rocket::response::content;

#[path = "./elastic/client.rs"]
mod client;

struct ElasticClient(client::Client);

#[get("/")]
fn index() -> &'static str {
    "Hello, netsle!"
}

#[get("/netstats")]
fn network_stats(elastic: State<ElasticClient>) -> content::Json<String> {
    let response = elastic.0.match_all_for_index("netsle");
    content::Json(response)
}

fn main() {
    let elastic = client::Client::new("elastic", "changeme");    
    rocket::ignite().manage(ElasticClient(elastic)).mount("/", routes![index, network_stats]).launch();
}