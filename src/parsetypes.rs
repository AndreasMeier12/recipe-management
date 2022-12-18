use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ParseRecipe {
    pub course: String,
    pub season: ESeason,
    pub book: String,
    pub name: String,
    pub page: Option<u16>,
    pub ingredients: Vec<String>,
}
impl fmt::Display for ParseRecipe{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "{} {} {} {} {:?}", self.course, self.season, self.book, self.name, self.page)
    }
}


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESeason {
    Summer,
    Winter,
    Autumn,
    Spring,
    Independent,
}

impl ESeason {
    pub fn value(a: ESeason) -> usize {
        match a {
            ESeason::Summer => 1,
            ESeason::Autumn => 2,
            ESeason::Winter => 3,
            ESeason::Spring => 4,
            ESeason::Independent => 5,
        }
    }

    pub fn value_rofl(&self) -> usize {
        return ESeason::value(*self);
    }

    pub fn get_seasons() -> Vec<ESeason> {
        return vec![ESeason::Summer, ESeason::Autumn, ESeason::Winter, ESeason::Spring, ESeason::Independent];
    }

    pub fn to_string<'a>(seas: &'a ESeason) -> &'a str {
        match seas {
            ESeason::Summer => "Summer",
            ESeason::Autumn => "Autumn",
            ESeason::Winter => "Winter",
            ESeason::Spring => "Spring",
            ESeason::Independent => "Independent",

        }



    }

    pub fn from_string(a: &str) -> Option<ESeason>{
        match a.to_lowercase().as_str() {
            "summer" => Some(ESeason::Summer),
                        "summer" => Some(ESeason::Summer),
            "summer" => Some(ESeason::Summer),
            "spring" => Some(ESeason::Spring),
            "independent" => Some(ESeason::Independent),
            _ => None
        }
    }

    pub fn to_map() -> HashMap<usize, ESeason> {
        return HashMap::from([(1, ESeason::Summer), (2, ESeason::Autumn), (3, ESeason::Winter), (4, ESeason::Spring), (5, ESeason::Independent)]);
    }

    pub fn get_by_db_id(id: i32) -> ESeason {
        return *(ESeason::to_map().get(&(id as usize)).unwrap());
    }
}


impl fmt::Display for ESeason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ESeason::Summer => write!(f, "summer"),
            ESeason::Spring => write!(f, "spring"),
            ESeason::Autumn => write!(f, "autumn"),
            ESeason::Winter => write!(f, "winter"),
            ESeason::Independent => write!(f, "independent"),
        }
    }
}


pub fn match_season(a: &str) -> ESeason {
    match a.to_lowercase().as_str(){
        "summer" => return ESeason::Summer,
        "autumn" => return ESeason::Autumn,
        "winter" => return ESeason::Winter,
        "spring" => return ESeason::Spring,
        _ => return ESeason::Independent,
    }
}

#[derive(Eq, Hash, PartialEq)]
pub struct FileWithCourse{
    pub filename: String,
    pub contents: String,
}