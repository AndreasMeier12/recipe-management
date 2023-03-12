use std::option::Option;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RecipePrefill {
    pub course: Option<i32>,
    pub book: Option<i32>,
    pub season: Option<usize>,
}

#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct SearchPrefill {
    pub name: Option<String>,
    pub season: Option<i32>,
    pub course: Option<i32>,
    pub book: Option<i32>,
    pub tried: Option<i32>,
}

impl SearchPrefill {
    pub fn template_name(&self)  -> String {
        self.name.as_ref().unwrap_or(&"".to_string()).clone()
    }
    pub fn empty(&self) -> bool{

        return self.season.is_none() && self.book.is_none() && self.season.is_none() && self.course.is_none();
    }

}