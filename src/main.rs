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
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket::http::Method;

#[database("main")]
pub struct MainDbConn(diesel::SqliteConnection);

fn main() {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Put, Method::Post, Method::Delete].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "content-type", "Refresh-Token"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().unwrap();


    // Load environment variables through the file .env
    dotenv::from_filename(".env").ok();

    let elastic = elastic::client::Client::new("elastic", "changeme");
    bootstrap_database();
    rocket::ignite()
        .manage(elastic::ElasticClient(elastic))
        .attach(MainDbConn::fairing())
        .attach(cors)
        .mount(
            "/",
            routes![
                routes::get::index,
                routes::get::dashboard_packet_count_graph,
                routes::get::dashboard_total_packets,
                routes::get::dashboard_ports_data,
                routes::get::dashboard_usage_data,
                routes::get::dashboard_hosts_data,
                routes::get::signout,
                routes::get::user_list,
                routes::post::login,
                routes::post::create_user,
                routes::post::refresh_token,
                routes::post::refresh_token_options,
                routes::delete::delete_user,
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
        database::users::create_user(&connection, "netsle".to_string(), "changeme".to_string(), true);

    match create_user_result {
        Ok(_) => {}
        Err(e) => panic!(e),
    }
    Some(())
}
