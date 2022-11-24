-- Your SQL goes here
CREATE TABLE IF NOT EXISTS ingredient
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       VARCHAR(255),
    created_at REAL DEFAULT (datetime('now', 'localtime'))
);

CREATE TABLE IF NOT EXISTS recipe_ingredient
(
    recipe_id     INTEGER NOT NULL,
    ingredient_id INTEGER NOT NULL,
    created_at    REAL DEFAULT (datetime('now', 'localtime')),
    PRIMARY KEY (recipe_id, ingredient_id),
    FOREIGN KEY (recipe_id) REFERENCES Recipe (recipe_id),
    FOREIGN KEY (recipe_id) REFERENCES ingredient (id)
);