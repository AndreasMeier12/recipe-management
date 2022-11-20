-- Your SQL goes here

CREATE TABLE IF NOT EXISTS Recipe(
    recipe_id INTEGER PRIMARY KEY AUTOINCREMENT,
    primary_season INTEGER NOT NULL,
    course INTEGER NOT NULL,
    book INTEGER,
    recipe_name VARCHAR(255),
    recipe_url VARCHAR(255),
    created_at REAL DEFAULT (datetime('now', 'localtime')),
    FOREIGN KEY (course) REFERENCES Course(course_id),
    FOREIGN KEY (book) REFERENCES Book(book_id),
    FOREIGN KEY (primary_season) REFERENCES Season(season_id)
);


CREATE TABLE IF NOT EXISTS Course(
    course_id INTEGER PRIMARY KEY AUTOINCREMENT ,
    course_name VARCHAR(255) UNIQUE,
    created_at REAL DEFAULT (datetime('now', 'localtime'))


);

CREATE TABLE IF NOT EXISTS Book(
    book_id INTEGER PRIMARY KEY AUTOINCREMENT,
    book_name VARCHAR(255) UNIQUE,
    created_at REAL DEFAULT (datetime('now', 'localtime'))


);

CREATE TABLE IF NOT EXISTS Season(
    season_id INTEGER PRIMARY KEY AUTOINCREMENT ,
    tag_name VARCHAR(255) UNIQUE,
    created_at REAL DEFAULT (datetime('now', 'localtime'))

);