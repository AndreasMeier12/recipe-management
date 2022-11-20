use std::fmt; 

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ParseRecipe {
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


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Season{
    Summer,
    Winter,
    Autumn,
    Spring,
    Independent,
}

impl Season {
    pub fn value(a: Season) -> usize{
        match a {
            Season::Summer =>1,
            Season::Autumn => 2,
            Season::Winter=>3,
            Season::Spring => 4,
            Season::Independent => 5,

        }
    }

    pub fn to_string<'a>(seas: &'a Season) -> &'a str{
        match seas {
            Season::Summer => "Summer",
            Season::Autumn => "Autumn",
            Season::Winter=>"Winter",
            Season::Spring => "Spring",
            Season::Independent => "Independent",

        }



    }

    pub fn from_string(a: &str) -> Option<Season>{
        match a.to_lowercase().as_str() {
            "summer" => Some(Season::Summer),
                        "summer" => Some(Season::Summer),
            "summer" => Some(Season::Summer),
            "spring" => Some(Season::Spring),
            "independent" => Some(Season::Independent),
            _ => None


        }

    }

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