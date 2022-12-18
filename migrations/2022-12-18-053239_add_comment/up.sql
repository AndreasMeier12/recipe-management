-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE recipe_comment
(
    comment_id INTEGER NOT NULL PRIMARY KEY,
    user_id    INTEGER NOT NULL,
    recipe_id  INTEGER NOT NULL,
    content    TEXT    NOT NULL,
    created_at REAL    NOT NULL DEFAULT (datetime('now', 'localtime')),
    FOREIGN KEY (recipe_id) REFERENCES Recipe (recipe_id),
    FOREIGN KEY (user_id) REFERENCES user (id)
);