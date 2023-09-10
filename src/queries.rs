use itertools::Itertools;

use crate::args::SearchPrefill;

pub fn build_search_query(params: &SearchPrefill, user_id: i32) -> String {
    let mut simple_criteria: Vec<String> = vec![];

    if params.book.filter(|x| *x >= 0).is_some() {
        simple_criteria.push(format!("book_id={}", params.book.unwrap()));
    }

    let seasons = handle_seasons(params);
    if seasons.is_some() {
        simple_criteria.push(seasons.unwrap());
    }

    if params.course.filter(|x| *x >= 0).is_some() {
        simple_criteria.push(format!("course_id={}", params.course.unwrap()))
    }

    if params.tried == 1 {
        simple_criteria.push(format!("SELECT * FROM recipe WHERE EXISTS(SELECT * FROM tried WHERE user_id={} and recipe.recipe_id=tried.recipe_id)", user_id));
    }
    if params.tried == 2 {
        simple_criteria.push(format!("SELECT * FROM recipe WHERE NOT EXISTS(SELECT * FROM tried WHERE user_id={} and recipe.recipe_id=tried.recipe_id)", user_id));
    }


    let name_for_real = params.name.as_ref().unwrap_or(&"".to_string()).trim().to_string()
        .replace("'", "''")
        .replace("_", "[_]")
        .replace("%", "[%]");
    if name_for_real.is_empty() {
        let args = simple_criteria.join("\nAND\n");
        return format!("SELECT * FROM RECIPE WHERE {}", args);
    }

    let asdf = format!("SELECT * FROM
(SELECT r.*
FROM ingredient
         INNER JOIN recipe_ingredient ri on ingredient.id = ri.ingredient_id
         INNER JOIN recipe r on r.recipe_id = ri.recipe_id
WHERE name LIKE '%{}%'
UNION SELECT r.* FROM recipe as r
               LEFT OUTER JOIN recipe_text rt on r.recipe_id = rt.recipe_id

               WHERE recipe_name LIKE '%{}%'
OR content LIKE '%{}%'
OR recipe_url LIKE '%{}%'
UNION SELECT r2.* FROM recipe_comment INNER JOIN recipe r2 on r2.recipe_id = recipe_comment.recipe_id
WHERE recipe_comment.content LIKE '%{}%')", name_for_real, name_for_real, name_for_real, name_for_real, name_for_real);
    if simple_criteria.is_empty() {
        return asdf;
    }
    let res = vec![asdf, simple_criteria.iter().join("AND")].iter().join(" WHERE ");
    return res;
}

fn handle_seasons( params: &SearchPrefill) -> Option<String>{
    let seasons = vec![params.season1, params.season2, params.season3, params.season4, params.season5];
    let search_seasons: Vec<String> = seasons.iter().enumerate()
        .filter(|(_i, x)| x.is_some())
        .map(|(i, _x)| (i+1).to_string())
        .collect();
    if search_seasons.is_empty(){
        return None;
    }

    return Some(format!("primary_season IN ({})", search_seasons.join(",")));



}

pub fn get_recipe_ids_with_comments() -> String {
    "SELECT DISTINCT recipe_id FROM recipe_comment; ".to_string()
}

pub fn get_recipe_ids_with_texts() -> String {
    "SELECT DISTINCT recipe_id FROM recipe_text;".to_string()
}