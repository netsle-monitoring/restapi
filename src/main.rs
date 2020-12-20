#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use dotenv;
mod elastic;
mod guards;
mod routes;
mod mongo;

fn main() {
    // Load environment variables through the file .env
    dotenv::from_filename(".env").ok();

    let elastic = elastic::client::Client::new("elastic", "changeme");
    let users = vec![(String::from("amit"), String::from("123"))];

    rocket::ignite()
        .manage(elastic::ElasticClient(elastic))
        .manage(guards::Users(users))
        .manage(mongo::connection::init_pool())
        .mount(
            "/",
            routes![
                routes::get::index,
                routes::get::network_stats,
                routes::post::login
            ],
        )
        .launch();
}
