use crate::crypto::hash_password;
use crate::database::models::{NewUser, User};
use crate::schema::users::dsl::{
    hashed_pw as hashed_pw_column, id as id_column, refresh_token as refresh_token_column,
    salt as salt_column, username as username_column, users,
};
use crate::MainDbConn;
use diesel::prelude::*;
use diesel::{insert_into, SqliteConnection};

pub fn create_user(conn: &SqliteConnection, username: String, password: String) {
    let (salt, hash) = hash_password(&username, &password);
    let new_user = NewUser {
        username: &username,
        refresh_token: "",
        hashed_pw: &hash,
        salt,
    };

    insert_into(users)
        .values(&new_user)
        .execute(&*conn)
        .unwrap();
    // diesel::insert_into(users).
}

// TODO: Find a way of not duplicating this piece of code.
pub fn get_user(conn: &SqliteConnection, username: &str) -> Option<User> {
    let result: Option<User> = users
        // .select((id_column, username_column, salt_column, refresh_token_column, hashed_pw_column))
        .filter(username_column.eq(username.to_string()))
        .first(&*conn)
        .optional()
        .unwrap();

    if (result.is_some()) {
        return result;
    } else {
        return None;
    }
}

pub fn get_username_for_refresh_token(conn: &SqliteConnection, token: &str) -> Option<String> {
    let result: Option<User> = users
        // .select((id_column, username_column, salt_column, refresh_token_column, hashed_pw_column))
        .filter(refresh_token_column.eq(token.to_string()))
        .first(&*conn)
        .optional()
        .unwrap();

    if (result.is_some()) {
        return Some(result.unwrap().username);
    } else {
        return None;
    }
}

pub fn update_refresh_token(conn: &SqliteConnection, username: &str, refresh_token: &str) {
    let target = users.filter(username_column.eq(username.to_string()));
    let result = diesel::update(target)
        .set(refresh_token_column.eq(refresh_token.to_string()))
        .execute(&*conn);
}
