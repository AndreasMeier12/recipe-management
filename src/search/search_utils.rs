use crate::args::SearchPrefill;
use crate::parsetypes::ESeason;
use crate::text_search::{SearchState, SCHEMA_BOOK};
use std::collections::HashMap;
use tantivy::query::{BooleanQuery, Occur, Query};
use tantivy::schema::Facet;
use tantivy::Term;

pub fn build_query(options: SearchPrefill, search_state: SearchState, book_names: HashMap<i32, String>, season_names: HashMap<usize, ESeason>, course_names: HashMap<i32, String>) -> BooleanQuery {
    let mut parts: Vec<(Occur, Box<dyn Query>)> = vec![];
    if let Some(name_query) = options.clone().name.filter(|x| !x.trim().is_empty()) {
        name_query.split(" ").into_iter().for_each(|x| parts.push(format!("+{}", x)));
    }

    if let Some(i) = book_names.get(&options.clone().book.unwrap_or(-1)) {
        let field = search_state.index.schema().get_field(SCHEMA_BOOK).expect("Book facet should exist in schema");

        let term = Term::from_facet(
            field,
            &Facet::from(format!("+book:/book/{}", i).to_string()),
        );
        parts.push((Occur::Should, Box::new(term)));
    }
    if let Some(season_term) = crate::text_search::build_season_term(options.clone(), season_names) {
        parts.push(season_term)
    }
    if let Some(i) = course_names.get(&options.course.unwrap_or(-1)) {
        parts.push(format!("+course:/course/{}", i))
    }

    return BooleanQuery::new(parts);
}