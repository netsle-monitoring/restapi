-- Your SQL goes here
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL, 
    hashed_pw TEXT NOT NULL, 
    refresh_token TEXT NOT NULL, 
    salt TEXT NOT NULL
 )