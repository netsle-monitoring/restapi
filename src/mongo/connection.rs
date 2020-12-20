use dotenv::dotenv;
use r2d2::PooledConnection;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::env;
use std::ops::Deref;

type Pool = r2d2::Pool<MongodbConnectionManager>;

pub struct Conn(pub PooledConnection<MongodbConnectionManager>);

/*
    create a connection pool of mongodb connections to allow a lot of users to modify db at same time.
*/
pub fn init_pool() -> Pool {
    dotenv().ok();
    let mongo_addr = env::var("DB_ADDR").expect("DB_ADDR must be set");
    let mongo_port = env::var("DB_PORT").expect("DB_PORT must be set");
    let db_name = env::var("DB_NAME").expect("DB_NAME env var must be set");
    let db_user = env::var("DB_USER").expect("DB_USER env var must be set");
    let db_pass = env::var("DB_PASS").expect("DB_PASS env var must be set");

    let manager = MongodbConnectionManager::new(
        ConnectionOptions::builder()
            .with_host(&mongo_addr, mongo_port.parse::<u16>().unwrap())
            .with_db(&db_name)
            .with_auth(&db_user, &db_pass)
            .build(),
    );
    
    match Pool::builder().max_size(16).build(manager) {
        Ok(pool) => pool,
        Err(e) => panic!("Error: failed to create mongodb pool {}", e),
    }
}

/*
    Create a implementation of FromRequest so Conn can be provided at every api endpoint
*/
impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(db) => Outcome::Success(Conn(db)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

/*
    When Conn is dereferencd, return the mongo connection.
*/
impl Deref for Conn {
    type Target = PooledConnection<MongodbConnectionManager>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}