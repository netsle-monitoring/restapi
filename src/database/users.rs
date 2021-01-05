use crate::database::models::{NewUser, User};
use crate::crypto::{hash_password, verify_password};
use crate::schema::users::dsl::{users,
    id as id_column,
    username as username_column,
    salt as salt_column,
    hashed_pw as hashed_pw_column,
    refresh_token as refresh_token_column
};
use crate::MainDbConn;
use diesel::{SqliteConnection, insert_into, update};
use diesel::prelude::*;
pub fn create_user(conn: &SqliteConnection, username: String, password: String) {
    let (salt, hash) = hash_password(&username, &password);
    let new_user = NewUser {
        username: &username,
        refresh_token: "",
        hashed_pw: &hash,
        salt
    };

    insert_into(users).values(&new_user).execute(&*conn).unwrap();
    // diesel::insert_into(users).
}

pub fn get_user(conn: &SqliteConnection, username: String) -> Option<User> {
    let result: Option<User> = users
        // .select((id_column, username_column, salt_column, refresh_token_column, hashed_pw_column))
        .filter(username_column.eq(username))
        .first(&*conn)
        .optional()
        .unwrap();
   
    if (result.is_some()) {
        return result;
    } else {
        return None;
    }
}

pub fn update_refresh_token(conn: &SqliteConnection, username: String, refresh_token: String) {
    let target = users.filter(username_column.eq(username));
    let result = diesel::update(target).set(refresh_token_column.eq(refresh_token)).execute(&*conn);

    println!("{:?}", result);
}