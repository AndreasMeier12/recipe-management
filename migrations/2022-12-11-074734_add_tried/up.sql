-- Your SQL goes here
CREATE TABLE tried
(
    user_id    INTEGER NOT NULL,
    recipe_id  INTEGER NOT NULL,
    created_at REAL DEFAULT (datetime('now', 'localtime')),
    FOREIGN KEY (recipe_id) REFERENCES Recipe (recipe_id),
    FOREIGN KEY (user_id) REFERENCES user (id),
    PRIMARY KEY (recipe_id, user_id)
);