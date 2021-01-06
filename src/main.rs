#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
use diesel::prelude::*;

use dotenv;
use std::env;

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
    bootstrap_database();
    rocket::ignite()
        .manage(elastic::ElasticClient(elastic))
        .attach(MainDbConn::fairing())
        .attach(guards::cors::CORS())
        .mount(
            "/",
            routes![
                routes::get::index,
                routes::get::network_stats,
                routes::post::login,
                routes::post::refresh_token,
                routes::post::refresh_token_options
            ],
        )
        .launch();
}

fn bootstrap_database() -> Option<()> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = diesel::SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url));

    let default_user = database::users::get_user(&connection, "netsle");

    if default_user.is_some() {
        println!("Database is already bootstrapped with the default user!");
        return None;
    }

    let create_user_result =
        database::users::create_user(&connection, "netsle".to_string(), "changeme".to_string());

    match create_user_result {
        Ok(_) => {}
        Err(e) => panic!(e),
    }
    Some(())
}
