#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod elastic;
mod routes;
mod guards;

fn main() {
    let elastic = elastic::client::Client::new("elastic", "changeme");   
    let users = vec!((String::from("amit"), String::from("123")));

    rocket::ignite()
        .manage(elastic::ElasticClient(elastic))
        .manage(guards::Users(users))
        .mount("/", routes![
            routes::get::index, 
            routes::get::network_stats, 
            routes::get::home
        ])
        .launch();
}