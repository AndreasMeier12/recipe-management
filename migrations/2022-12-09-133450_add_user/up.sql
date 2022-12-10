-- Your SQL goes here

CREATE TABLE user
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    email      VARCHAR(255) NOT NULL UNIQUE,
    pw_hash    VARCHAR(255) NOT NULL,
    created_at REAL DEFAULT (datetime('now', 'localtime'))


)