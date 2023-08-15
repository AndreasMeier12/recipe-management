use serde::Deserialize;

#[derive(Deserialize)]
pub struct RecipePrefill {
    pub course: Option<i32>,
    pub book: Option<i32>,
    pub season: Option<usize>,
}

#[derive(Deserialize)]
#[derive(Default)]
pub struct SearchPrefill {
    pub name: Option<String>,
    pub season: Option<i32>,
    pub course: Option<i32>,
    pub book: Option<i32>,
    pub tried: i32,
    pub season1: Option<i32>,
    pub season2: Option<i32>,
    pub season3: Option<i32>,
    pub season4: Option<i32>,
    pub season5: Option<i32>,


}

impl SearchPrefill {
    pub fn template_name(&self)  -> String {
        self.name.as_ref().unwrap_or(&"".to_string()).clone()
    }
}