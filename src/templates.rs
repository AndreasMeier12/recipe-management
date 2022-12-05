use std::collections::HashMap;

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
pub struct BookForm {
    // in your template
}


impl<'a> BookForm {
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
    pub recipes: Option<Vec<FullRecipe>>

    // in your template
}

impl<'a> SearchForm<'a> {
    pub fn get(&self) -> String {
        return self.render().unwrap();
    }
}
