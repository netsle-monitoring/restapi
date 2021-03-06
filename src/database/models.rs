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
    pub is_admin: bool
}

#[derive(Debug, Queryable, Serialize)]
pub struct BlacklistEntry {
    pub ip: String
}

#[derive(Debug, Queryable, Serialize)]
pub struct PublicUser {
    pub id: Option<i32>,
    pub username: String,
    pub is_admin: bool
}


#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub hashed_pw: &'a [u8],
    pub refresh_token: &'a str,
    pub salt: Vec<u8>,
    pub is_admin: bool
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "blacklist"]
pub struct NewBlacklistEntry<'a> {
    pub ip: &'a str
}
