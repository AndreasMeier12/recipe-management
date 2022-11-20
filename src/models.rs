use diesel::prelude::*;
use super::schema::Recipe;
use super::schema::Course;
use super::schema::Season;
use super::schema::Book;



#[derive(Queryable)]
#[diesel(table_name = Recipe)]
pub struct FullRecipe {
    pub recipe_id: Option<i32>,
    pub primary_season: i32,
    pub course: i32,
    pub book: Option<i32>,
    pub page: Option<i32>,
    pub recipe_name: String,
    pub recipe_url: Option<String>,
}

impl FullRecipe {
    pub fn new(recipe_id: Option<i32>, primary_season: i32, course: i32, book: Option<i32>, recipe_name: String, recipe_url: Option<String>, page: Option<i32>) -> FullRecipe {
        FullRecipe { recipe_id: recipe_id, recipe_name: recipe_name, book: book, course: course, primary_season: primary_season, recipe_url: recipe_url, page: page }
    }
}

#[derive(Insertable)]
#[diesel(table_name = Recipe)]
pub struct InsertRecipe {
    pub primary_season: i32,
    pub course: i32,
    pub book: Option<i32>,
    pub recipe_name: String,
    pub page: Option<i32>
}


#[derive(Queryable)]
#[diesel(table_name = Course)]
pub struct QCourse {
    pub course_id: Option<i32>,
    pub course_name: String,
}


#[derive(Insertable)]
#[diesel(table_name = Course)]
pub struct InsertCourse {
    pub course_id: Option<i32>,
    pub course_name: String,
}

impl InsertCourse {
pub  fn new(course_id: Option<i32>, course_name: String) -> InsertCourse {
    InsertCourse {course_id: course_id, course_name: course_name}
    }
}

impl QCourse {
pub  fn new(course_id: Option<i32>, course_name: String) -> QCourse {
    QCourse {course_id: course_id, course_name: course_name}
    }
}


#[derive(Queryable, Clone, Debug)]
#[diesel(table_name = Book)]
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
#[diesel(table_name = Book)]
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
#[diesel(table_name = Season)]
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
#[diesel(table_name = Season)]
pub struct InsertSeason {
    pub season_id: Option<i32>,
    pub tag_name: String,
}

impl InsertSeason {
    pub fn new(season_id: Option<i32>, name: String) -> InsertSeason {
        InsertSeason { season_id: season_id, tag_name: name }
    }
}