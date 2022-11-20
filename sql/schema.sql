CREATE TABLE IF NOT EXISTS Recipe(
    recipe_id INTEGER NOT NULL,
    primary_season INTEGER NOT NULL,
    course INTEGER NOT NULL,
    book INTEGER,
    page INTEGER,
    recipe_name VARCHAR(255),
    recipe_url VARCHAR(255),
)

CREATE TABLE IF NOT EXISTS Course(
    course_id INTEGER PRIMARY KEY,
    course_name VARCHAR(255)

)

CREATE TABLE IF NOT EXISTS Book(
    book_id INTEGER PRIMARY KEY,
    book_name VARCHAR(255)

)

CREATE TABLE IF NOT EXISTS Season(
    season_id INTEGER PRIMARY KEY,
    tag_name VARCHAR(255), 
)

CREATE TABLE IF NOT EXISTS Tag(
    tag_id INTEGER PRIMARY KEY,
    tag_name VARCHAR(255), 
)

CREATE TABLE IF NOT EXISTS RecipeTag(
    recipe_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL
)