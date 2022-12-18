-- Your SQL goes here

CREATE TABLE recipe_text
(
    recipe_id   INTEGER NOT NULL PRIMARY KEY,
    content     TEXT    not null,
    created_at  REAL    NOT NULL DEFAULT (datetime('now', 'localtime')),
    modified_at REAL,
    FOREIGN KEY (recipe_id) REFERENCES Recipe (recipe_id)
)