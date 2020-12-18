#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod elastic;
mod routes;

fn main() {
    let elastic = elastic::client::Client::new("elastic", "changeme");   

    rocket::ignite().manage(elastic::ElasticClient(elastic)).mount("/", routes![
        routes::get::index, 
        routes::get::network_stats, 
        routes::get::home
    ])
    .launch();
}