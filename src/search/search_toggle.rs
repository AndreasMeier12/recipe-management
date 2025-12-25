use std::collections::HashMap;

use crate::args::SearchPrefill;
use crate::models::{FullRecipe, QBook, QCourse};
use crate::parsetypes::ESeason;
use crate::queries::{build_index_search_query, build_search_query};
use crate::schema::book::dsl::book;
use crate::schema::course::dsl::course;
use crate::text_search::{build_query, SCHEMA_BODY, SCHEMA_INGREDIENTS, SCHEMA_RECIPE_ID, SCHEMA_TITLE};
use diesel::{sql_query, RunQueryDsl, SqliteConnection};
use diesel_logger::LoggingConnection;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::Value;
use tantivy::{Index, TantivyDocument};

pub fn search(search_args: &SearchPrefill, con: &mut LoggingConnection<SqliteConnection>, index: &Index, user_id: i32) -> Vec<FullRecipe> {
    let sql_string: String = if search_args.legacy.filter(|x| x.clone() == 1).is_some() { build_search_query(&search_args, user_id) } else { build_tantivy_search_for_sql(search_args, con, index, user_id) };


    let recipes = sql_query(sql_string)
        .load::<FullRecipe>(con)
        .ok().unwrap_or(vec![]);
    return recipes;
}

fn build_tantivy_search_for_sql(search_args: &SearchPrefill, con: &mut LoggingConnection<SqliteConnection>, index: &Index, user_id: i32) -> String {
    let reader = index.reader().unwrap();
    let query_parser = QueryParser::for_index(&index, vec![index.schema().get_field(SCHEMA_TITLE).unwrap(), index.schema().get_field(SCHEMA_INGREDIENTS).unwrap(), index.schema().get_field(SCHEMA_BODY).unwrap()]);

    use crate::schema::book::dsl::*;
    let books: HashMap<i32, String> = book.load::<QBook>(con).unwrap()
        .iter()
        .map(|x| (x.clone().book_id.unwrap(), x.clone().book_name.unwrap()))
        .collect();


    let seasons: HashMap<usize, ESeason> = ESeason::to_map();
    use crate::schema::course::dsl::*;

    let course_names: HashMap<i32, String> = course.load::<QCourse>(con).unwrap()
        .iter()
        .map(|x| (x.clone().course_id.unwrap(), x.course_name.clone().unwrap()))
        .collect();


    let query_string = build_query(search_args.clone(), books, seasons, course_names);
    let parse_res = query_parser.parse_query(query_string.as_str());
    let query = query_parser.parse_query(query_string.as_str()).unwrap();
    let searcher = reader.searcher();
    let results = searcher.search(&query, &TopDocs::with_limit(1024));
    let index_recipes: Vec<TantivyDocument> = results.unwrap().iter().map(|x| searcher.doc(x.1))
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect();
    let recipe_ids: Vec<i64> = index_recipes.iter().map(|x| x.get_first(index.schema().get_field(SCHEMA_RECIPE_ID).expect("We might have a problem, recipes should always have ids")))
        .map(|x| x.expect("Id should have valu\
        e"))
        .map(|x| x.as_i64().unwrap())
        .collect();
    return build_index_search_query(recipe_ids, search_args, user_id);
}



