use std::fmt; 

#[derive(Debug, Clone)]
pub struct ParseRecipe{
    pub course: String,
    pub season: Season,
    pub book: String,
    pub name: String,
    pub page: u16,
    pub ingredients: Vec<String>,
}
impl fmt::Display for ParseRecipe{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "{} {} {} {} {}", self.course, self.season, self.book, self.name, self.page)
    }
}


#[derive(Debug, Copy, Clone)]
pub enum Season{
    Summer,
    Winter,
    Autumn,
    Spring,
    Independent,
}


impl fmt::Display for Season {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Season::Summer => write!(f, "summer"),
            Season::Spring => write!(f, "spring"),
            Season::Autumn => write!(f, "autumn"),
            Season::Winter => write!(f, "winter"),
            Season::Independent => write!(f, "independent"),
        }
    }
}


pub fn match_season(a: &str) -> Season{
    match a.to_lowercase().as_str(){
        "summer" => return Season::Summer,
        "autumn" => return Season::Autumn,
        "winter" => return Season::Winter,
        "spring" => return Season::Spring,
        _ => return Season::Independent,
    }
}