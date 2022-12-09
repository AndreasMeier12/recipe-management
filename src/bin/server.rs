use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::net::SocketAddr;
use std::ops::Deref;

use axum::{Form, Router, routing::get};
use axum::{body::Body, response::{Html, Json}};
use axum::extract::{Path, Query};
use axum::response::Redirect;
use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::BoolOrNullableBool;
use itertools::Itertools;
use serde::Deserialize;

use recipemanagement::*;
use recipemanagement::args::{RecipePrefill, SearchPrefill};
use recipemanagement::models::*;
use recipemanagement::parsetypes::ESeason;
use recipemanagement::schema::course::dsl::course;
use recipemanagement::templates::*;

#[tokio::main]
async fn main() {
    // Route all requests on "/" endpoint to anonymous handler.
    //
    // A handler is an async function which returns something that implements
    // `axum::response::IntoResponse`.

    // A closure or a function can be used as handler.

    let app = Router::new().route("/", get(index_handler))
        .route("/course/:name", get(handle_course))
        .route("/book/add", get(book_form).post(post_book))
        .route("/recipe/add", get(recipe_form).post(post_recipe))
        .route("/search", get(search_form).post(search_result))
        ;
    //        Router::new().route("/", get(|| async { "Hello, world!" }));

    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Use `hyper::server::Server` which is re-exported through `axum::Server` to serve the app.
    axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index_handler() -> Html<String> {
    let con = &mut database::establish_connection();
    use recipemanagement::schema::course::dsl::*;
    let courses: Vec<QCourse> = course.load::<QCourse>(con).unwrap();
    let course_refs: &Vec<QCourse> = &courses;
    let hello = HelloTemplate { name: "world", courses: course_refs.clone(), title: "roflcopter" }; // instantiate your struct
    let a = hello.get();

    return Html(a);
}

async fn handle_course(Path(path): Path<String>) -> Html<String> {
    let name = path.as_str();
    let con = &mut database::establish_connection();
    use recipemanagement::schema::course::dsl::*;
    let reses = &course
        .filter(course_name.eq(name))
        .load::<QCourse>(con)
        .unwrap();
    let res = &reses
        .first()
        .map(|x| x.course_id)
        .unwrap().unwrap();

    let asdf: &QCourse = reses.first().unwrap();
    use recipemanagement::schema::recipe::dsl::*;
    let a: Option<i32> = None;



    use recipemanagement::schema::recipe::dsl::*;
    let recipes: Vec<FullRecipe> = recipe.filter(recipemanagement::schema::recipe::course_id.eq(res)).load::<FullRecipe>(con).unwrap();
    use recipemanagement::schema::book::dsl::*;

    let books: Vec<QBook> = book.load::<QBook>(con).unwrap();
    let id_to_book: HashMap<i32, QBook> = books.iter().map(|x| (x.book_id.unwrap(), x.clone())).collect();
    let season_map = ESeason::to_map();
    let recipes_per_book_season: HashMap<(usize, i32), Vec<&FullRecipe>> = recipes.iter()
        .map(|x| ((x.primary_season as usize, x.book_id.unwrap()), x))
        .into_group_map();
    let courses: Vec<QCourse> = course.load::<QCourse>(con).unwrap();
    let course_refs: &Vec<QCourse> = &courses;

    let content = CourseTemplate { course_name: asdf.course_name.as_ref().unwrap().as_str(), seasons: ESeason::get_seasons(), recipes_per_book_season: recipes_per_book_season, books: &books, courses: course_refs, title: name }.get();

    return Html(content);
}


async fn recipe_form(prefill: Query<RecipePrefill>) -> Html<String> {
    let con = &mut database::establish_connection();

    use recipemanagement::schema::book::dsl::*;

    let books: Vec<QBook> = book.load::<QBook>(con).unwrap();
    use recipemanagement::schema::course::dsl::*;

    let courses: Vec<QCourse> = course.load::<QCourse>(con).unwrap();
    let course_refs: &Vec<QCourse> = &courses;


    return Html(RecipeForm { seasons: ESeason::get_seasons(), books: &books, courses: course_refs, prefill: prefill.0, title: "Add Recipe" }.get());
}

#[derive(Deserialize)]
struct PostRecipe {
    course: i32,
    book: Option<String>,
    season: i32,
    name: String,
    url: Option<String>,
    page: Option<String>,

}

async fn post_recipe(Form(form): Form<PostRecipe>) -> Redirect {
    let con = &mut database::establish_connection();
    use recipemanagement::schema::recipe;
    let book_id = form.book.map(|x| x.parse::<i32>()).unwrap_or(Ok(0)).ok();
    let page = form.page.map(|x| x.parse::<i32>()).unwrap_or(Ok(0)).ok();

    let recipe = InsertRecipe { recipe_id: None, recipe_name: form.name, primary_season: form.season, course_id: form.course, book_id: book_id, page: page };

    diesel::insert_into(recipe::table)
        .values(vec![recipe])
        .execute(con)
        .unwrap();
    let url = format!("/recipe/add?season={}&course={}&book={}", form.season, form.course, book_id.unwrap_or(0));


    return Redirect::to(url.as_str());
}


#[derive(Deserialize)]
struct PostBook {
    booktext: String,
}

async fn book_form() -> Html<String> {
    let con = &mut database::establish_connection();

    let courses: Vec<QCourse> = course.load::<QCourse>(con).unwrap();
    let course_refs: &Vec<QCourse> = &courses;


    return Html(BookForm { courses: course_refs, title: "Add book" }.get());
}

async fn post_book(Form(form): Form<PostBook>) -> Redirect {
    let content = form;
    let con = &mut database::establish_connection();
    {
        use recipemanagement::schema::book::dsl::*;

        let books: Vec<QBook> = book.load::<QBook>(con).unwrap();
        if books.iter().any(|x| x.book_name.as_ref().filter(|y| **y == content.booktext).is_some()) {
            return Redirect::to("/recipe/add?val=error");
        }
    }
    use recipemanagement::schema::book;
    use crate::schema::book::book_name;

    diesel::insert_into(book::table)
        .values(book_name.eq(content.booktext))
        .execute(con)
        .unwrap();


    return Redirect::to("/");
}

#[derive(Deserialize)]
struct SearchRecipe {
    course: Option<i32>,
    book: Option<i32>,
    season: Option<i32>,
    name: Option<String>,
}

async fn search_form() -> Html<String>{
    let con = &mut database::establish_connection();

    use recipemanagement::schema::book::dsl::*;

    let books: Vec<QBook> = book.load::<QBook>(con).unwrap();
    use recipemanagement::schema::course::dsl::*;

    let courses: Vec<QCourse> = course.load::<QCourse>(con).unwrap();
    let course_refs: &Vec<QCourse> = &courses;


    return Html(SearchForm { seasons: ESeason::get_seasons(), books: &books, courses: course_refs, recipes: None, title: "Search" }.get());

}

async fn search_result(Form(form): Form<SearchRecipe>) -> Html<String>{
        let con = &mut database::establish_connection();

    use recipemanagement::schema::book::dsl::*;

    let books: Vec<QBook> = book.load::<QBook>(con).unwrap();
    use recipemanagement::schema::course::dsl::*;

    let courses: Vec<QCourse> = course.load::<QCourse>(con).unwrap();
    let course_refs: &Vec<QCourse> = &courses;

        use recipemanagement::schema::recipe::dsl::*;
    let mut recipe_query = recipe.into_boxed();
    if form.course.as_ref().filter(|x| **x != 0).is_some() {
        recipe_query = recipe_query.filter(recipemanagement::schema::recipe::course_id.eq(form.course.unwrap()))
    }
    if form.season.as_ref().filter(|x| **x != 0).is_some() {
        recipe_query = recipe_query.filter(recipemanagement::schema::recipe::primary_season.eq(form.season.unwrap()))
    }
    if form.book.as_ref().filter(|x| **x != 0).is_some() {
        recipe_query = recipe_query.filter(recipemanagement::schema::recipe::book_id.eq(form.book.unwrap()))
    }

    let has_name = form.name.as_ref().filter(|x| **x != "").is_some();

    if has_name {
        let arg = format!("%{}%", form.name.as_ref().unwrap());
        recipe_query = recipe_query.filter(recipemanagement::schema::recipe::recipe_name.like(arg))
    }
    let mut recipes = recipe_query.load::<FullRecipe>(con).unwrap();
    if has_name {
        let sql_string = format!("SELECT r.*
FROM ingredient
         INNER JOIN recipe_ingredient ri on ingredient.id = ri.ingredient_id
         INNER JOIN recipe r on r.recipe_id = ri.recipe_id
WHERE name LIKE '{}'", form.name.as_ref().unwrap());

        let moar_recipes = sql_query(sql_string)
            .load::<FullRecipe>(con);
        if moar_recipes.is_ok() {
            recipes.extend(moar_recipes.unwrap())
        }
    }


    return Html(SearchForm { seasons: ESeason::get_seasons(), books: &books, courses: course_refs, recipes: Some(recipes), title: "Search" }.get());
}

fn query_course() {}

