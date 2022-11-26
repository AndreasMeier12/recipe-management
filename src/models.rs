use diesel::prelude::*;
use diesel::sql_types::{Double, Float, Timestamp};
use itertools::Format;

use super::schema::book;
use super::schema::course;
use super::schema::ingredient;
use super::schema::recipe;
use super::schema::recipe_ingredient;
use super::schema::season;

#[derive(Queryable, Associations)]
#[diesel(table_name = recipe)]
#[belongs_to(parent = "QCourse", foreign_key = "course_id")]
pub struct FullRecipe {
    pub recipe_id: Option<i32>,
    pub primary_season: i32,
    pub course_id: i32,
    pub book_id: Option<i32>,
    pub recipe_name: Option<String>,
    pub recipe_url: Option<String>,
    pub created_at: Option<f32>,
    pub page: Option<i32>,

}


#[derive(Insertable)]
#[diesel(table_name = recipe)]
pub struct InsertRecipe {
    pub primary_season: i32,
    pub course_id: i32,
    pub book_id: Option<i32>,
    pub recipe_name: String,
    pub page: Option<i32>,
    pub recipe_id: Option<i32>,
}


#[derive(Queryable)]
#[diesel(table_name = course)]
pub struct QCourse {
    pub course_id: Option<i32>,
    pub course_name: Option<String>,
    pub created_at: Option<f32>,
}


#[derive(Insertable)]
#[diesel(table_name = course)]
pub struct InsertCourse {
    pub course_id: Option<i32>,
    pub course_name: String,
}

impl InsertCourse {
pub  fn new(course_id: Option<i32>, course_name: String) -> InsertCourse {
    InsertCourse {course_id: course_id, course_name: course_name}
    }
}



#[derive(Queryable, Clone, Debug)]
#[diesel(table_name = book)]
pub struct QBook {
    pub book_id: Option<i32>,
    pub book_name: String,
}

impl QBook {
    pub fn new(book_id: Option<i32>, name: String) -> QBook {
        QBook { book_id: book_id, book_name: name }
    }
}


#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = book)]
pub struct InsertBook {
    pub book_id: Option<i32>,
    pub book_name: String,
}

impl InsertBook {
    pub fn new(book_id: Option<i32>, name: String) -> InsertBook {
        InsertBook { book_id: book_id, book_name: name }
    }
}

#[derive(Queryable)]
#[diesel(table_name = season)]
pub struct QSeason {
    pub season_id: Option<i32>,
    pub name: String,
}


impl QSeason {
    pub fn new(season_id: Option<i32>, name: String) -> QSeason {
        QSeason { season_id: season_id, name: name }
    }
}

#[derive(Insertable)]
#[diesel(table_name = season)]
pub struct InsertSeason {
    pub season_id: Option<i32>,
    pub tag_name: String,
}

impl InsertSeason {
    pub fn new(season_id: Option<i32>, name: String) -> InsertSeason {
        InsertSeason { season_id: season_id, tag_name: name }
    }
}

#[derive(Queryable)]
#[diesel(table_name = ingredient)]
pub struct Ingredient {
    pub id: Option<i32>,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = ingredient)]
pub struct InsertIngredient {
    pub id: Option<i32>,
    pub name: String,
}

#[derive(Queryable)]
#[diesel(table_name = recipe_ingredient)]
pub struct RecipeIngredient {
    pub recipe_id: i32,
    pub ingredient_id: i32,
}


#[derive(Insertable)]
#[diesel(table_name = recipe_ingredient)]
pub struct InsertRecipeIngredient {
    pub recipe_id: i32,
    pub ingredient_id: i32,
}

impl InsertRecipeIngredient {
    pub fn to_string(&self) -> String {
        format!("recipe: {}, tag {}", self.recipe_id, self.ingredient_id)
    }
}