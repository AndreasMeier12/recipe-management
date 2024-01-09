use diesel::{RunQueryDsl, sql_query, SqliteConnection};
use diesel_logger::LoggingConnection;
use tantivy::{Document, Index};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;

use crate::args::SearchPrefill;
use crate::models::FullRecipe;
use crate::queries::{build_index_search_query, build_search_query};
use crate::text_search::SCHEMA_RECIPE_ID;

pub fn search(search_args: &SearchPrefill, con: &mut LoggingConnection<SqliteConnection>, index: &Index, user_id: i32) -> Vec<FullRecipe> {
    let reader = index.reader().unwrap();
    let query_parser = QueryParser::for_index(&index, vec![index.schema().get_field("title").unwrap()]);
    let query = query_parser.parse_query(&*format!("{}", search_args.clone().name.unwrap_or("".to_string()))).unwrap();
    let searcher = reader.searcher();
    let results = searcher.search(&query, &TopDocs::with_limit(1024));
    let index_recipes: Vec<Document> = results.unwrap().iter().map(|x| searcher.doc(x.1))
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect();
    let recipe_ids: Vec<i64> = index_recipes.iter().map(|x| x.get_first(index.schema().get_field(SCHEMA_RECIPE_ID).expect("We might have a problem, recipes should always have ids")))
        .map(|x| x.expect("Id should have value"))
        .map(|x| x.as_i64().unwrap())
        .collect();
    let query_string: String = if search_args.legacy.filter(|x| x.clone() == 1).is_some() { build_search_query(&search_args, user_id) } else { build_index_search_query(recipe_ids) };


    let recipes = sql_query(query_string)
        .load::<FullRecipe>(con)
        .ok().unwrap_or(vec![]);
    return recipes;
}


