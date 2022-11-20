use diesel::prelude::*;
use super::schema::Recipe;

#[derive(Queryable)]
#[diesel(table_name = Recipe)]
pub struct FullRecipe {
    pub recipe_id: Option<i32>,
    pub primary_season: i32,
    pub course: i32,
    pub book: Option<i32>,
    pub recipe_name: String,
    pub recipe_url: Option<String>,
}

impl FullRecipe {
    pub fn new(recipe_id: Option<i32>, primary_season: i32, course: i32, book: Option<i32>, recipe_name: String, recipe_url: Option<String>) -> FullRecipe {
        FullRecipe { recipe_id: recipe_id, recipe_name: recipe_name, book: book, course: course, primary_season: primary_season, recipe_url: recipe_url }
    }
}

#[derive(Insertable)]
#[diesel(table_name = Recipe)]
pub struct InsertRecipe {
    pub primary_season: i32,
    pub course: i32,
    pub book: Option<i32>,
    pub recipe_name: String,
}


#[derive(Queryable)]
#[diesel(table_name = Course)]
pub struct Course {
    pub course_id: Option<i32>,
    pub course_name: String,
}


impl Course{
pub  fn new(course_id: Option<i32>, course_name: String) -> Course{
    Course{course_id: course_id, course_name: course_name}
    }
}


#[derive(Queryable, Clone, Debug)]
#[diesel(table_name = Book)]
pub struct Book {
    pub book_id: Option<i32>,
    pub book_name: String,
}


impl Book {
    pub fn new(book_id: Option<i32>, name: String) -> Book {
        Book { book_id: book_id, book_name: name }
    }
}

#[derive(Queryable)]
#[diesel(table_name = Season)]
pub struct Season {
    pub season_id: Option<i32>,
    pub name: String,
}


impl Season {
    pub fn new(season_id: Option<i32>, name: String) -> Season {
        Season { season_id: season_id, name: name }
    }
}