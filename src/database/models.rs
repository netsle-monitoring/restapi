/* Import macros and others */
use crate::schema::*;

/* For beeing able to serialize */
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: Option<i32>, 
    pub username: String,
    pub hashed_pw: Vec<u8>,
    pub refresh_token: String,
    pub salt: Vec<u8>,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub hashed_pw: &'a [u8],
    pub refresh_token: &'a str,
    pub salt: Vec<u8>,
}