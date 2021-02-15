use crate::schema::blacklist::dsl::{
     ip as ip_column
};

use diesel::prelude::*;
use crate::database::models::{NewBlacklistEntry, BlacklistEntry};
use diesel::{insert_into, SqliteConnection};
use crate::schema::blacklist;

pub fn create_entry(conn: &SqliteConnection, ip: String) -> std::result::Result<(), &'static str> {
    let result = diesel::insert_into(blacklist::table).values(&NewBlacklistEntry {
        ip: &ip
    }).execute(&*conn);

    if result.is_err() {
        return Err("The entry already exists");
    }

    Ok(())
}

pub fn delete_entry(conn: &SqliteConnection, ip: String) -> std::result::Result<(), &'static str> {
    let result = diesel::delete(blacklist::table.filter(ip_column.eq(&ip))).execute(&*conn);

    if result.is_err() {
        return Err("Unknown error occured");
    }

    Ok(())
}

pub fn get_all_entries(conn: &SqliteConnection) -> Option<Vec<String>> {
    let result = blacklist::table.select(ip_column).load::<String>(&*conn);

    if result.is_ok() {
        return Some(result.unwrap());
    } else {
        return None;
    }
}
