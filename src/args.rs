use serde::Deserialize;

#[derive(Deserialize)]
pub struct RecipePrefill {
    pub course: Option<i32>,
    pub book: Option<i32>,
    pub season: Option<usize>,
}