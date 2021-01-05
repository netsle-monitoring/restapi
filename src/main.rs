#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

use dotenv;

mod crypto;
mod database;
mod elastic;
mod guards;
mod routes;
mod schema;

#[database("main")]
pub struct MainDbConn(diesel::SqliteConnection);

fn main() {
    // Load environment variables through the file .env
    dotenv::from_filename(".env").ok();

    let elastic = elastic::client::Client::new("elastic", "changeme");
    let users = vec![(String::from("amit"), String::from("123"))];

    rocket::ignite()
        .manage(elastic::ElasticClient(elastic))
        .manage(guards::Users(users))
        .attach(MainDbConn::fairing())
        .mount(
            "/",
            routes![
                routes::get::index,
                routes::get::network_stats,
                routes::post::login,
                routes::post::refresh
            ],
        )
        .launch();
}
