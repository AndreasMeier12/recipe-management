use std::collections::{HashMap, HashSet};

use askama::Template;

use crate::args::RecipePrefill;
use crate::models::*;
use crate::parsetypes::ESeason;

// bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "hello.html")] // using the template in this path, relative
// to the `templates` dir in the crate root
pub struct HelloTemplate<'a> {
    // the name of the struct can be anything
    pub name: &'a str,
    pub courses: &'a Vec<QCourse>,
    // the field name should match the variable name
    pub title: &'a str,
    pub user_id: Option<i32>,
    // in your template
}


impl<'a> HelloTemplate<'a> {
    pub fn get(&self) -> String {
        return self.render().unwrap();
    }
}

#[derive(Template)] // this will generate the code...
#[template(path = "course.html")] // using the template in this path, relative
// to the `templates` dir in the crate root
pub struct CourseTemplate<'a> {
    pub courses: &'a Vec<QCourse>,
    pub books: &'a Vec<QBook>,
    pub course_name: &'a str,
    pub seasons: Vec<ESeason>,
    pub recipes_per_book_season: HashMap<(usize, i32), Vec<&'a FullRecipe>>,
    pub title: &'a str,
    pub tried: HashSet<i32>,
    pub logged_in: bool,
    pub recipes_to_ingredients: HashMap<i32, Vec<String>>,
    pub user_id: Option<i32>,

    // in your template
}


impl<'a> CourseTemplate<'a> {
    pub fn get(&self) -> String {
        return self.render().unwrap();
    }
}

#[derive(Template)] // this will generate the code...
#[template(path = "add_book.html")] // using the template in this path, relative
// to the `templates` dir in the crate root
pub struct BookForm<'a> {
    pub courses: &'a Vec<QCourse>,
    pub title: &'a str,
    pub user_id: Option<i32>,

    // in your template
}


impl<'a> BookForm<'a> {
    pub fn get(&self) -> String {
        return self.render().unwrap();
    }
}

#[derive(Template)] // this will generate the code...
#[template(path = "add_recipe.html")] // using the template in this path, relative
pub struct RecipeForm<'a> {
    pub courses: &'a Vec<QCourse>,
    pub books: &'a Vec<QBook>,
    pub seasons: Vec<ESeason>,
    pub prefill: RecipePrefill,
    pub title: &'a str,
    pub newest: String,
    pub user_id: Option<i32>,


    // in your template
}




impl<'a> RecipeForm<'a> {
    pub fn get(&self) -> String {
        return self.render().unwrap();
    }
}

#[derive(Template)] // this will generate the code...
#[template(path = "search_form.html")] // using the template in this path, relative
pub struct SearchForm<'a> {
    pub courses: &'a Vec<QCourse>,
    pub books: &'a Vec<QBook>,
    pub seasons: Vec<ESeason>,
    pub recipes: Option<Vec<FullRecipe>>,
    pub title: &'a str,
    pub recipes_to_ingredients: HashMap<i32, Vec<String>>,
    pub user_id: Option<i32>,

    // in your template
}

impl<'a> SearchForm<'a> {
    pub fn get(&self) -> String {
        return self.render().unwrap();
    }
}

#[derive(Template)] // this will generate the code...
#[template(path = "login.html")] // using the template in this path, relative
pub struct LoginPage<'a> {
    pub courses: &'a Vec<QCourse>,
    pub title: &'a str,
    pub user_id: Option<i32>,

    // in your template
}


impl<'a> LoginPage<'a> {
    pub fn get(&self) -> String {
        return self.render().unwrap();
    }
}

#[derive(Template)] // this will generate the code...
#[template(path = "recipe_edit_form.html")] // using the template in this path, relative
pub struct RecipeEditForm<'a> {
    pub courses: &'a Vec<QCourse>,
    pub recipe: &'a FullRecipe,
    pub ingredients: String,
    pub title: &'a str,
    pub books: &'a Vec<QBook>,
    pub seasons: Vec<ESeason>,
    pub prefill_season: usize,
    pub recipe_text: String,
    pub user_id: Option<i32>,
    // in your template
}


impl<'a> RecipeEditForm<'a> {
    pub fn get(&self) -> String {
        return self.render().unwrap();
    }
}

mod filters {
    pub fn asref<'a, T>(s: &'a T) -> askama::Result<&'a T> { //https://github.com/djc/askama/issues/330
        Ok(s)
    }
}

#[derive(Template)] // this will generate the code...
#[template(path = "recipe_detail.html")] // using the template in this path, relative
pub struct RecipeDetail<'a> {
    pub courses: &'a Vec<QCourse>,
    pub course: &'a str,
    pub recipe: &'a FullRecipe,
    pub ingredients: Vec<String>,
    pub title: &'a str,
    pub book_name: &'a Option<String>,
    pub season: ESeason,
    pub tried: bool,
    pub comments: Vec<Comment>,
    pub recipe_text: String,
    pub user_id: Option<i32>,
}

impl<'a> RecipeDetail<'a> {
    pub fn get(&self) -> String {
        return self.render().unwrap();
    }
}