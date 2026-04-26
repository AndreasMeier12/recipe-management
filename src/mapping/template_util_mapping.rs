use crate::models::FullRecipe;
use crate::templates::DisplayFullRecipe;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn mapRecipeForSearch(recipe: &FullRecipe, commented: &HashSet<i32>, tried_ids: &HashSet<i32>, texted: &HashSet<i32>, book_names: &HashMap<i32, String>, ingredients: &HashMap<i32, Vec<String>>) -> DisplayFullRecipe {
    let book_name: Option<String> = map_book_name(recipe, book_names);
    let ingredients = ingredients.get(&recipe.recipe_id.expect("Should exist"))
        .map(|x| x.clone());

    return DisplayFullRecipe {
        recipe: recipe.clone(),
        season_name: "".to_string(),
        book_name: book_name,
        url: recipe.recipe_url.clone(),
        tried: tried_ids.contains(&recipe.recipe_id.unwrap()),
        ingredients,
        texted: texted.contains(&recipe.recipe_id.unwrap()),
        commented: commented.contains(&recipe.recipe_id.unwrap()),
    };
}

fn map_book_name(recipe: &FullRecipe, book_names: &HashMap<i32, String>) -> Option<String> {
    if recipe.book_id.is_none() {
        return None;
    }
    let mut parts: Vec<String> = vec! {};
    parts.push(book_names.get(&recipe.book_id.expect("We checked before")).expect("Referenced book should be found").clone());
    if let Some(page_id) = recipe.page {
        parts.push(page_id.to_string())
    }

    return Some(parts.iter().join(": "))
}