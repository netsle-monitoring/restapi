/* Import macros and others */
use crate::schema::*;

/* For beeing able to serialize */
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32, 
    pub username: String,
    pub hashed_pw: String,
    pub refresh_token: String,
    pub salt: String,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub hashed_pw: String,
    pub refresh_token: String,
    pub salt: String,
}