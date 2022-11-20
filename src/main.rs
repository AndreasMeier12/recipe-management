use std::path::{Path};
use std::{fs, vec};
use std::collections::HashMap;
use std::ffi::OsStr;

use parsetypes::ParseRecipe;
use parsetypes::Season;
use crate::Season::Independent;
mod parsetypes;
use regex::Regex;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use itertools::Itertools;
use crate::models::{Book, Course, FullRecipe, InsertRecipe};


pub mod models;
pub mod schema;



fn main() {
    let in_path_file = Path::new("path.txt");
    let in_path = fs::read_to_string(in_path_file)
    .expect("There really should be a path");
    let in_path_os = OsStr::new(&in_path);

    let path = Path::new(in_path_os);
    let raw_contents = fs::read_to_string(path)
    .expect("Could not open file");

    let res =  parse(raw_contents);
    let books: Vec<Book> = res.iter().map(|x| x.book.as_str())
        .unique()
        .enumerate()
        .map(|(i,x)| models::Book::new(Some(i as i32), x.to_string()))
        .collect();
    let book_title_to_id: HashMap<_, _> = books.iter().map(|x| (&x.book_name, x.book_id))
        .collect();
    let courses: Vec<Course> = res.iter().map(|x| x.course.as_str())
        .unique()
        .enumerate()
        .map(|(i,x)| Course::new(Some(i as i32), x.to_string()))
        .collect();
    let courses_to_id: HashMap<_, _> = courses.iter()
        .map(|x| (x.course_name.as_str(), x.course_id))
        .collect();
    let recipes: Vec<_> = res.iter()
        .enumerate()
        .map(|(i, x)| {
            let bookd_id: Option<i32> = *book_title_to_id.get(&x.book).unwrap() ;
            let season_id = Season::value(x.season) as i32;
            let name = x.name.clone();
            let course_id = courses_to_id.get(x.course.as_str()).unwrap().unwrap();
            return models::InsertRecipe{course: course_id, book: bookd_id, recipe_name: name, primary_season: season_id}
        })
        .collect();

    let con = &mut establish_connection();

    use crate::schema::Recipe;
    diesel::insert_into(Recipe::table)
        .values(&recipes)
        .execute(con)
        .unwrap();



    print!("{:?}", books)



}


fn parse(raw_contents: String) -> Vec<ParseRecipe>{


    let split_contents = raw_contents.lines()
    .filter(|x| !x.is_empty())
    .filter(|x| x.matches('\t').count() <= 2)
    .filter(|x| !x.replace('\t', "").is_empty());
    let mut res : Vec<ParseRecipe> = Vec::new();
    let mut book: String = String::from("");
    let mut season: Season = Independent;

    for a in split_contents{
        let depth = a.matches('\t').count();
        let b = a.replace('\t', "");

        match depth {
            0 => {
                season = parsetypes::match_season(b.as_str());
            }
            1 => {
                book = b;

            }
            2 => {
                let name: String = parse_recipe_name(b.clone()).to_string();
                let ingredients = parse_ingredients(b.clone()); 
                let asdf = ParseRecipe{
                    course: String::from("bread"),
                    name,
                    season,
                    page: parse_page_number(b.clone()),
                    book: book.to_string(),
                    ingredients 


                };
                res.push(asdf);

            }
            _ => {
                panic!("this level should not exist\n \"{}\"", a);
            }
            
        }


    }
    return res;

}




fn parse_recipe_name(b: String) -> String{
    if b.contains('[') {
        return b.split('[').next().expect("Should be here").trim().to_string();
    }
    let re = Regex::new(r"/\d+$/").unwrap();

    return re.replace_all(&b[..], "").to_string();
}

fn parse_ingredients(b: String) -> Vec<String> {
    if b.contains(']') && b.contains('[') {
        let c = b.split(']').next().expect("Should be here").split('[').nth(1).expect("Should be here");
        return c.split('\n').map(str::to_string).collect();
    }
    return vec![];



}

fn parse_page_number(b: String) -> u16{
    let c = b.split(' ').last().unwrap();
    c.parse::<u16>().expect("This should be a positive number!")

}



pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}