use std::{fs, vec};
use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::OsStr;
use std::fs::read_to_string;
use std::path::Path;

use diesel::prelude::*;
use diesel::result::Error;
use dotenvy::dotenv;
use itertools::Itertools;
use regex::Regex;

use parsetypes::ESeason;
use parsetypes::ParseRecipe;

use crate::ESeason::Independent;
use crate::models::{FullRecipe, InsertBook, InsertCourse, InsertIngredient, InsertRecipe, InsertRecipeIngredient, InsertSeason, QBook, QCourse};
use crate::parsetypes::FileWithCourse;

mod parsetypes;

pub mod models;
pub mod schema;


fn main() {
    let in_path_file = Path::new("path.txt");
    let read_in = read_in(in_path_file);

    let res: Vec<ParseRecipe> = read_in.into_iter().map(|x| parse(x.contents, x.filename))
        .flatten().collect();

    let books: Vec<InsertBook> = res.iter().map(|x| x.book.as_str())
        .unique()
        .enumerate()
        .map(|(i, x)| models::InsertBook::new(Some(i as i32), x.to_string()))
        .collect();
    let book_title_to_id: HashMap<_, _> = books.iter().map(|x| (&x.book_name, x.book_id))
        .collect();
    let courses: Vec<InsertCourse> = res.iter().map(|x| x.course.as_str())
        .unique()
        .enumerate()
        .map(|(i, x)| InsertCourse::new(Some(i as i32), x.to_string()))
        .collect();
    let courses_to_id: HashMap<_, _> = courses.iter()
        .map(|x| (x.course_name.as_str(), x.course_id))
        .collect();
    let recipes: Vec<_> = res.iter()
        .enumerate()
        .map(|(i, x)| {
            let bookd_id: Option<i32> = *book_title_to_id.get(&x.book).unwrap();
            let season_id = ESeason::value(x.season) as i32;
            let name = x.name.clone();
            let course_id = courses_to_id.get(x.course.as_str()).unwrap().unwrap();
            let page = x.page.map(|x| x as i32);
            return models::InsertRecipe { course: course_id, book: bookd_id, recipe_name: name, primary_season: season_id, page: page, recipe_id: Some(i as i32) };
        })
        .collect();

    let insert_seasons: Vec<InsertSeason> = build_seasons_records();
    let recipe_name_to_id: HashMap<_, _> = recipes.iter().map(|x| (&x.recipe_name, x.recipe_id)).collect();

    let ingredient_infos = build_tag_records(res, recipe_name_to_id);

/*    let diagnostic_out = tag_stuff.1.iter().map(|x| x.to_string()).join("\n");
    let out_path = Path::new("out_recipe_tags.txt");
    fs::write(out_path, diagnostic_out).unwrap();
*/
    let con = &mut establish_connection();

    use crate::schema::recipe;
    let transaction_res = con.transaction::<_, Error, _>(|x| {
        diesel::insert_into(recipe::table)
            .values(&recipes)
            .execute(x)
            .unwrap();
        use crate::schema::course;
        diesel::insert_into(course::table)
            .values(&courses)
            .execute(x)
            .unwrap();
        use crate::schema::book;
        diesel::insert_into(book::table)
            .values(&books)
            .execute(x)
            .unwrap();
        use crate::schema::season;
        diesel::insert_into(season::table)
            .values(&insert_seasons)
            .execute(x)
            .unwrap();
        use crate::schema::ingredient;
        diesel::insert_into(ingredient::table)
            .values(&ingredient_infos.0)
            .execute(x)
            .unwrap();
        use crate::schema::recipe_ingredient;
        diesel::insert_into(recipe_ingredient::table)
            .values(&ingredient_infos.1)
            .execute(x)
            .unwrap();
        Ok(())
    });


    print!("{:?}", books)
}

fn build_tag_records(a: Vec<ParseRecipe>, recipe_name_to_id: HashMap<&String, Option<i32>>) -> (Vec<InsertIngredient>, Vec<InsertRecipeIngredient>) {
    let tags: Vec<InsertIngredient> = a.iter()
        .map(|x| &x.ingredients)
        .flatten()
        .unique()
        .enumerate()
        .map(|(i, x)| InsertIngredient { name: x.to_string(), id: Some(i as i32) })
        .collect();
    let tag_name_to_id: HashMap<_, _> = tags.iter().map(|x| (&x.name, x.id.unwrap()))
        .collect();
    let recipe_tags = a.iter().map(|x| x.ingredients.iter().map(|y| InsertRecipeIngredient { ingredient_id: *tag_name_to_id.get(y).unwrap(), recipe_id: recipe_name_to_id.get(&x.name).unwrap().unwrap() }))
        .flatten().collect();


    return (tags, recipe_tags);
}

fn build_recipe_tags(a: ParseRecipe, b: HashMap<String, i32>){


}

fn build_seasons_records() -> Vec<InsertSeason> {
    return vec![build_season_record(ESeason::Summer),
                build_season_record(ESeason::Autumn),
                build_season_record(ESeason::Winter),
                build_season_record(ESeason::Spring),
                build_season_record(ESeason::Independent),
    ];
}

fn build_season_record(a: ESeason) -> InsertSeason {
    return InsertSeason::new(Some(ESeason::value(a) as i32), ESeason::to_string(&a).to_string());
}

fn read_in(configPath: &Path) -> Vec<FileWithCourse> {
    let in_paths: Vec<String> = fs::read_to_string(configPath)
        .expect("There really should be a path").split("\n").into_iter()
        .map(|x| x.to_string())
        .collect();
    let mut seen_courses: HashSet<String> = HashSet::new();
    let mut res: Vec<FileWithCourse> = vec![];
    for in_path in in_paths {
        let in_path_os = OsStr::new(&in_path);
        let path = Path::new(in_path_os);

        let course = path.file_name().unwrap().to_str().unwrap().replace(".txt", "").to_string();
        if !seen_courses.contains(course.as_str()) {
            let raw_contents = fs::read_to_string(path)
                .expect("Could not open file");

            seen_courses.insert(course.to_string());

            res.push(FileWithCourse { contents: raw_contents, filename: course.to_string() })
        }
    }
    return res;
}


fn parse(raw_contents: String, course: String) -> Vec<ParseRecipe> {
    let split_contents = raw_contents.lines()
        .filter(|x| !x.is_empty())
        .filter(|x| x.matches('\t').count() <= 2)
        .filter(|x| !x.replace('\t', "").is_empty());
    let mut res: Vec<ParseRecipe> = Vec::new();
    let mut book: String = String::from("");
    let mut season: ESeason = Independent;

    for a in split_contents {
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
                let ingredients: Vec<String> = parse_ingredients(b.clone()).iter().map(|x| x.to_string()).unique().collect();
                let asdf = ParseRecipe {
                    course: course.clone(),
                    name,
                    season,
                    page: parse_page_number(b.clone()),
                    book: book.to_string(),
                    ingredients,
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


fn parse_recipe_name(b: String) -> String {
    let re = Regex::new(r"/\d+$/").unwrap();
    let c = re.replace(b.as_str(), "");
    if c.contains('[') {
        return b.split('[').next().expect("Should be here").trim().to_string();
    }
    let re = Regex::new(r"/\d+$/").unwrap();

    return re.replace_all(&c[..], "").to_string();
}

fn parse_ingredients(b: String) -> Vec<String> {
    if b.contains(']') && b.contains('[') {
        let c = b.split(']').next().expect("Should be here").split('[').nth(1).expect("Should be here");
        return c.split(", ").map(str::to_string).collect();
    }
    return vec![];
}

fn parse_page_number(b: String) -> Option<u16> {
    let c = b.split(' ').last().unwrap();
    let d = c.parse::<u16>();
    match d {
        Ok(T) => return Some(T),
        Err(T) => None
    }
}


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}