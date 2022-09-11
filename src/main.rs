use std::path::{Path};
use std::{fs, vec};
use std::ffi::OsStr;

use parsetypes::ParseRecipe;
use parsetypes::Season;
use crate::Season::Independent;
mod parsetypes;
use regex::Regex;


fn main() {
    let inPathFile = Path::new("path.txt");
    let inPath = fs::read_to_string(inPathFile)
    .expect("There really should be a path");
    let inPathOs = OsStr::new(&inPath);

    let path = Path::new(inPathOs);
    let rawContents = fs::read_to_string(path)
    .expect("Could not open file");

    let res =  parse(rawContents);
    for a in res  {
        println!("{}", a)
        
    }




}


fn parse(rawContents: String) -> Vec<ParseRecipe>{


    let splitContents = rawContents.lines()
    .filter(|x| !x.is_empty())
    .filter(|x| x.matches('\t').count() <= 2)
    .filter(|x| !x.replace("\t", "").is_empty());
    let mut res : Vec<ParseRecipe> = Vec::new();
    let mut book: String = String::from("");
    let mut season: Season = Independent;

    for a in splitContents{
        let depth = a.matches('\t').count();
        let b = a.replace("\t", "");

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
                    name: name,
                    season: season,
                    page: parse_page_number(b.clone()),
                    book: book.to_string(),
                    ingredients: ingredients 


                };
                res.push(asdf);

            }
            _ => {
                panic!("this level should not exist");
            }
            
        }


    }
    return res;

}




fn parse_recipe_name(b: String) -> String{
    if b.contains("[") {
        return b.split("[").nth(0).expect("Should be here").trim().to_string();
    }
    let re = Regex::new(r"/\d+$/").unwrap();

    return re.replace_all(&b[..], "").to_string();
}

fn parse_ingredients(b: String) -> Vec<String> {
    if b.contains("]") && b.contains("[") {
        let c = b.split("]").nth(0).expect("Should be here").split("[").nth(1).expect("Should be here");
        return c.split("\n").map(str::to_string).collect();
    }
    return vec![];



}

fn parse_page_number(b: String) -> u16{
    let c = b.split(" ").last().unwrap();
    c.parse::<u16>().expect("This should be a positive number!")

}
