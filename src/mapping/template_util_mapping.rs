use crate::models::FullRecipe;
use crate::templates::DisplayFullRecipe;
use std::collections::{HashMap, HashSet};

pub fn mapRecipeForSearch(recipe: &FullRecipe, commented: &HashSet<i32>, tried_ids: &HashSet<i32>, texted: HashSet<i32>, book_names: &HashMap<i32, String>, ingredients: &HashMap<i32, Vec<String>>) -> DisplayFullRecipe {
    let book_name: Option<String> = recipe.book_id.map(|x| book_names.get(&x))
        .map();

    return DisplayFullRecipe {
        recipe: recipe.clone(),
        season_name: "".to_string(),
        book_name: book_name,
        url: recipe.recipe_url.clone(),
        tried: tried_ids.contains(&recipe.recipe_id.unwrap()),
        ingredients: ingredients.,
        texted: texted.contains(&recipe.recipe_id.unwrap()),
        commented: commented.contains(&recipe.recipe_id.unwrap()),
    };
}