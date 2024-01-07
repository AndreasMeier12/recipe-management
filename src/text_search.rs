use std::collections::HashMap;

use diesel::{Queryable, QueryableByName};
use itertools::Itertools;
use tantivy::Index;
use tantivy::schema::{FacetOptions, IndexRecordOption, Schema, TextFieldIndexing, TextOptions};
use tantivy::tokenizer::{AsciiFoldingFilter, LowerCaser, SimpleTokenizer, TextAnalyzer};

use crate::args::SearchPrefill;
use crate::models::{FullRecipe, Ingredient};
use crate::parsetypes::ESeason;

#[derive(Clone)]
pub struct SearchState {
    index: Index,
}

pub fn setup_search_state() -> tantivy::Result<SearchState> {
    let schema = build_schema();


    let index = Index::builder().schema(schema.clone()).create_from_tempdir()?;
    let mut tokenizer = TextAnalyzer::builder(SimpleTokenizer::default())
        .filter(LowerCaser)
        .filter(AsciiFoldingFilter)
        .build();
    index.tokenizers()
        .register("ascii", tokenizer);
    return Ok(SearchState {
        index,
    });
}

fn build_schema() -> Schema {
    let mut schema_builder = Schema::builder();

    let text_field_indexing = TextFieldIndexing::default()
        .set_tokenizer("ascii")
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);
    let text_options = TextOptions::default()
        .set_indexing_options(text_field_indexing)
        .set_stored();
    schema_builder.add_text_field("title", text_options.clone());
    schema_builder.add_text_field("body", text_options.clone());
    schema_builder.add_text_field("url", text_options.clone());
    schema_builder.add_facet_field("book", FacetOptions::default());
    schema_builder.add_facet_field("season", FacetOptions::default());
    schema_builder.add_facet_field("course", FacetOptions::default());
    schema_builder.build()
}


pub fn add_recipes(index: Index, recipes: Vec<(FullRecipe, Vec<Ingredient>)>) {}

pub fn search() {}

fn build_query(options: SearchPrefill, book_names: HashMap<i32, String>, season_names: HashMap<usize, ESeason>, course_names: HashMap<i32, String>) -> String {
    let mut parts: Vec<String> = vec![];
    if options.name.is_some() {
        parts.push(format!("{}", options.clone().name.unwrap()))
    }

    if let Some(i) = book_names.get(&options.clone().book.unwrap_or(-1)) {
        parts.push(format!("+book:{}", i))
    }
    if let Some(season_term) = build_season_term(options.clone(), season_names) {
        parts.push(season_term)
    }
    if let Some(i) = course_names.get(&options.course.unwrap_or(-1)) {
        parts.push(format!("+book:{}", i))
    }

    return parts.join(" ");

}


fn build_season_term(options: SearchPrefill, season_names: HashMap<usize, ESeason>) -> Option<String> {
    let raw_vals: Vec<Option<i32>> = vec![options.season1, options.season2, options.season3, options.season4, options.season5];

    let season_names: Vec<String> = raw_vals.iter().enumerate()
        .filter(|x| x.1.is_some())
        .map(|x| x.0 + 1)
        .map(|x| x)
        .map(|x| season_names.get(&(x)))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .map(|x| x.to_string())
        .collect();
    if season_names.is_empty() {
        return None;
    }

    let inner: String = season_names.into_iter().join(" ");
    return Some(format!("+season in [{}]", inner));
}

fn create_document() {}

pub fn nuke_and_rebuild_index() {}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use crate::args::SearchPrefill;
    use crate::parsetypes::ESeason;
    use crate::text_search::build_season_term;

    #[test]
    fn test_season_empty() {
        let options: SearchPrefill = SearchPrefill {
            name: None,
            season: None,
            course: None,
            book: None,
            tried: 0,
            season1: None,
            season2: None,
            season3: None,
            season4: None,
            season5: None,
        };
        let season_names = ESeason::to_map();
        let res = build_season_term(options, season_names);
        assert_eq!(None, res);
    }

    #[test]
    fn test_all_seasons() {
        let seasons = ESeason::get_seasons();
        let options: SearchPrefill = SearchPrefill {
            name: None,
            season: None,
            course: None,
            book: None,
            tried: 0,
            season1: Some(1),
            season2: Some(1),
            season3: Some(1),
            season4: Some(1),
            season5: Some(1),
        };
        let season_names = ESeason::to_map();
        let res = build_season_term(options, season_names);
        assert!(res.is_some());
        assert_equal("+season in [summer autumn winter spring independent]".to_string().bytes(), res.unwrap().bytes());
    }

    #[test]
    fn test_some_seasons() {
        let seasons = ESeason::get_seasons();
        let options: SearchPrefill = SearchPrefill {
            name: None,
            season: None,
            course: None,
            book: None,
            tried: 0,
            season1: Some(1),
            season2: None,
            season3: Some(1),
            season4: Some(1),
            season5: None,
        };
        let season_names = ESeason::to_map();
        let res = build_season_term(options, season_names);
        assert!(res.is_some());
        assert_equal("+season in [summer winter spring]".to_string().bytes(), res.unwrap().bytes());
    }
}