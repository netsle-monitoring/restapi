-- Your SQL goes here
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL, 
    hashed_pw BINARY NOT NULL, 
    refresh_token TEXT NOT NULL, 
    salt BINARY NOT NULL,
    is_admin BOOLEAN NOT NULL
 )