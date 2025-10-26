use std::collections::HashMap;
use std::sync::Arc;

use itertools::Itertools;
use tantivy::schema::{Facet, FacetOptions, IndexRecordOption, Schema, TextFieldIndexing, TextOptions, STORED};
use tantivy::tokenizer::{AsciiFoldingFilter, Language, LowerCaser, SimpleTokenizer, Stemmer, TextAnalyzer};
use tantivy::{Document, Index, IndexWriter};
use tokio::sync::Mutex;

use crate::args::SearchPrefill;
use crate::parsetypes::ESeason;
use crate::queries::RecipeQueryResult;
use crate::search::stop_words::StopFilter;
use crate::search::synonym_tokenizer::SynonymFilter;


#[derive(Clone)]
pub struct SearchState {
    pub index: Index,
    writer: Arc<Mutex<IndexWriter>>
}

pub fn setup_search_state() -> tantivy::Result<SearchState> {
    let schema = build_schema();


    let index = Index::builder().schema(schema.clone()).create_from_tempdir()?;
    let tokenizer = TextAnalyzer::builder(SimpleTokenizer::default())
        .filter(LowerCaser)
        .filter(StopFilter)
        .filter(AsciiFoldingFilter)
        .filter(Stemmer::new(Language::English))
        .filter(SynonymFilter)
        .build();
    index.tokenizers()
        .register("ascii", tokenizer);
    return Ok(SearchState {
        index: index.clone(),
        writer: Arc::new(Mutex::new(index.clone().writer(INDEX_MEMORY).unwrap()))

    });
}

pub const SCHEMA_TITLE: &'static str = "title";

pub const SCHEMA_BODY: &'static str = "body";

const SCHEMA_URL: &'static str = "url";

const SCHEMA_BOOK: &'static str = "book";

const SCHEMA_SEASON: &'static str = "season";

const SCHEMA_COURSE: &'static str = "course";

pub const SCHEMA_RECIPE_ID: &'static str = "recipe_id";

pub const SCHEMA_INGREDIENTS: &'static str = "ingredients";


fn build_schema() -> Schema {
    let mut schema_builder = Schema::builder();

    let text_field_indexing = TextFieldIndexing::default()
        .set_tokenizer("ascii")
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);
    let text_options = TextOptions::default()
        .set_indexing_options(text_field_indexing)
        .set_stored();
    schema_builder.add_text_field(SCHEMA_TITLE, text_options.clone());
    schema_builder.add_text_field(SCHEMA_BODY, text_options.clone());
    schema_builder.add_text_field(SCHEMA_INGREDIENTS, text_options.clone());
    schema_builder.add_text_field(SCHEMA_URL, text_options.clone());
    schema_builder.add_text_field(SCHEMA_RECIPE_ID, STORED);
    schema_builder.add_facet_field(SCHEMA_BOOK, FacetOptions::default());
    schema_builder.add_facet_field(SCHEMA_SEASON, FacetOptions::default());
    schema_builder.add_facet_field(SCHEMA_COURSE, FacetOptions::default());
    schema_builder.build()
}


const INDEX_MEMORY: usize = 50_000_000;


pub fn nuke_and_rebuild_with_recipes(search_state: &SearchState, recipes: Vec<RecipeQueryResult>) {
    let schema = search_state.index.schema();
    let mut index_writer = futures::executor::block_on(search_state.writer.lock());
    index_writer.delete_all_documents().expect("Writer access should be there");
    let season_ids_to_seasons = ESeason::to_map();
    for enriched_recipe in recipes {
        let mut doc = Document::default();
        if let Some(i) = enriched_recipe.recipe.recipe_name {
            doc.add_text(schema.get_field(SCHEMA_TITLE).unwrap(), i);
        }
        for ingredient_name in enriched_recipe.ingredients {
            doc.add_text(schema.get_field(SCHEMA_INGREDIENTS).unwrap(), ingredient_name);
        }
        doc.add_i64(schema.get_field(SCHEMA_RECIPE_ID).unwrap(), enriched_recipe.recipe.recipe_id.unwrap() as i64);
        doc.add_facet(schema.get_field(SCHEMA_COURSE).unwrap(), Facet::from(format!("/course/{}", enriched_recipe.course_name).as_str()));


        if let Some(name_string) = enriched_recipe.book_name {
            doc.add_facet(schema.get_field(SCHEMA_BOOK).unwrap(), Facet::from(format!("/book/{}", name_string.as_str()).as_str()));
        }

        if let Some(i) = enriched_recipe.recipe_text {
            doc.add_text(schema.get_field(SCHEMA_BODY).unwrap(), i);
        }
        let season_name = season_ids_to_seasons.get(&(enriched_recipe.recipe.primary_season as usize)).map(|x| x.to_string()).unwrap();
        doc.add_facet(schema.get_field(SCHEMA_SEASON).unwrap(), Facet::from(format!("/season/{}", season_name.as_str()).as_str()));

        index_writer.add_document(doc).expect("Writing should still work");
    }
    index_writer.commit().expect("Commit should work");

}

pub fn search() {}

pub fn build_query(options: SearchPrefill, book_names: HashMap<i32, String>, season_names: HashMap<usize, ESeason>, course_names: HashMap<i32, String>) -> String {
    let mut parts: Vec<String> = vec![];
    if let Some(name_query) = options.clone().name.filter(|x| !x.trim().is_empty()) {
        name_query.split(" ").into_iter().for_each(|x| parts.push(format!("+{}", x)));
    }

    if let Some(i) = book_names.get(&options.clone().book.unwrap_or(-1)) {
        parts.push(format!("+book:/book/{}", i))
    }
    if let Some(season_term) = build_season_term(options.clone(), season_names) {
        parts.push(season_term)
    }
    if let Some(i) = course_names.get(&options.course.unwrap_or(-1)) {
        parts.push(format!("+course:/course/{}", i))
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
        .map(|x| format!("/season/{}", x.to_string()))
        .collect();
    if season_names.is_empty() {
        return None;
    }

    let inner: String = season_names.into_iter().join(" ");
    return Some(format!("+season: IN [{}]", inner));
}


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
            legacy: None,
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
            legacy: None,
        };
        let season_names = ESeason::to_map();
        let res = build_season_term(options, season_names);
        assert!(res.is_some());
        assert_equal("+season: IN [/season/summer /season/autumn /season/winter /season/spring /season/independent]".to_string().bytes(), res.unwrap().bytes());
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
            legacy: None,
        };
        let season_names = ESeason::to_map();
        let res = build_season_term(options, season_names);
        assert!(res.is_some());
        assert_equal("+season: IN [/season/summer /season/winter /season/spring]".to_string().bytes(), res.unwrap().bytes());
    }
}