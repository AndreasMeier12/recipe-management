use itertools::Itertools;

use crate::args::SearchPrefill;

pub fn build_search_query(params: &SearchPrefill, user_id: i32) -> String {
    let mut simple_criteria: Vec<String> = vec![];

    if params.book.filter(|x| *x >= 0).is_some() {
        simple_criteria.push(format!("book_id={}", params.book.unwrap()));
    }
    if params.season.filter(|x| *x >= 0).is_some() {
        simple_criteria.push(format!("primary_season={}", params.season.unwrap()))
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