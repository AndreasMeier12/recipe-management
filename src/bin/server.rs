#[macro_use]
extern crate log;

use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::RandomState;
use std::fmt::format;
use std::net::SocketAddr;
use std::ops::Deref;

use argon2::{
    Argon2,
    password_hash::{
        PasswordHash,
        PasswordHasher, PasswordVerifier, rand_core::OsRng, SaltString,
    },
};
use axum::{Extension, Form, Router, routing::{get, post}};
use axum::{body::Body, response::{Html, Json}};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum_sessions::{async_session::MemoryStore, extractors::{ReadableSession, WritableSession}, Session, SessionLayer};
use axum_sessions::async_session::blake3::Hash;
use axum_sessions::async_session::blake3::IncrementCounter::No;
use axum_sessions::async_session::log::trace;
use diesel::{select, sql_query};
use diesel::dsl::{exists, max, sql};
use diesel::internal::operators_macro::FieldAliasMapper;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sql_types::{BoolOrNullableBool, Integer, Text};
use diesel_logger::LoggingConnection;
use env_logger::Env;
use itertools::Itertools;
use rand::Rng;
use regex::Regex;
use serde::Deserialize;

use recipemanagement::*;
use recipemanagement::args::{RecipePrefill, SearchPrefill, SearchRecipe};
use recipemanagement::models::*;
use recipemanagement::parsetypes::ESeason;
use recipemanagement::queries::build_search_query;
use recipemanagement::schema::course::dsl::course;
use recipemanagement::schema::ingredient::dsl::ingredient;
use recipemanagement::schema::recipe::primary_season;
use recipemanagement::schema::recipe_ingredient::recipe_id;
use recipemanagement::strops::extract_domain;
use recipemanagement::templates::*;

#[tokio::main]
async fn main() {
    // Route all requests on "/" endpoint to anonymous handler.
    //
    // A handler is an async function which returns something that implements
    // `axum::response::IntoResponse`.

    // A closure or a function can be used as handler.
    let store = MemoryStore::new();
    let secret = rand::thread_rng().gen::<[u8; 128]>();
    let session_layer = SessionLayer::new(store, &secret);

    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "debug")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);


    let app = Router::new().route("/", get(index_handler))
        .route("/course/:name", get(handle_course))
        .route("/book/add", get(book_form).post(post_book))
        .route("/recipe/add", get(recipe_form).post(post_recipe))
        .route("/search", get(search_form).post(search_result))
        .route("/login", get(login_page).post(my_login))
        .route("/recipe/edit/:id", get(edit_recipe_form).post(put_recipe))
        .route("/api/tried/:id", post(toggle_tried))
        .route("/recipe/detail/:id", get(recipe_detail).post(post_comment))
        .layer(session_layer)
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

async fn index_handler(session: ReadableSession) -> Html<String> {
    let con = &mut database::establish_connection();
    let maybe_user_id: Option<i32> = session.get::<i32>("user_id");
    use recipemanagement::schema::course::dsl::*;
    let courses: Vec<QCourse> = course.load::<QCourse>(con).unwrap();
    let course_refs: &Vec<QCourse> = &courses;
    let hello = HelloTemplate {
        name: "world",
        courses: course_refs.clone(),
        title: "Recipes",
        user_id: maybe_user_id,
    }; // instantiate your struct
    let a = hello.get();

    return Html(a);
}

async fn handle_course(session: ReadableSession, Path(path): Path<String>) -> Html<String> {
    let cur_name = path.as_str();
    let con = &mut database::establish_connection();
    use recipemanagement::schema::course::dsl::*;
    let reses = &course
        .filter(course_name.eq(cur_name))
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

    let books: Vec<QBook> = book.load::<QBook>(con).unwrap().into_iter().sorted_by(|x, y| x.book_name.as_ref().unwrap().cmp(y.book_name.as_ref().unwrap())).collect();
    let courses: Vec<QCourse> = course.load::<QCourse>(con).unwrap();
    let course_refs: &Vec<QCourse> = &courses;

    let mut tried_ids: HashSet<i32> = HashSet::new();
    let maybe_user_id: Option<i32> = session.get::<i32>("user_id");
    if maybe_user_id.is_some() {
        use recipemanagement::schema::tried::dsl::*;
        let temp = tried.filter(user_id.eq(maybe_user_id.unwrap()))
            .load::<Tried>(con)
            .unwrap();
        tried_ids = HashSet::from_iter(temp.iter().map(|x| x.recipe_id));
    }

    use recipemanagement::schema::ingredient::dsl::*;
    let id_to_ingredients: HashMap<i32, String> = ingredient.load::<Ingredient>(con)
        .unwrap()
        .iter()
        .map(|x| (x.clone().id.unwrap(), x.name.clone().unwrap()))
        .collect();

    use recipemanagement::schema::recipe_ingredient::dsl::*;
    let recipes_to_ingredients = recipe_ingredient.load::<RecipeIngredient>(con)
        .unwrap()
        .iter()
        .map(|x| (x.recipe_id, id_to_ingredients.get(&x.ingredient_id)))
        .filter(|x| x.clone().1.is_some())
        .map(|x| (x.clone().0, x.clone().1.unwrap().clone()))
        .into_group_map();

    let external_web_recipes: Vec<FullRecipe> = recipes.iter()
        .filter(|x| x.book_id.as_ref().is_none() && x.recipe_url.as_ref().is_some())
        .map(|x| x.clone())
        .collect();
    let web_pages: HashSet<String> = HashSet::from_iter(external_web_recipes.iter()
        .map(|x| extract_domain(x.recipe_url.clone().unwrap())));
    let mut recipes_by_season_and_source: Vec<(ESeason, Vec<(String, Vec<FullRecipe>)>)> = vec![];
    for season in ESeason::get_seasons() {
        let mut vals: Vec<(String, Vec<FullRecipe>)> = vec![];
        for lol_book in books.clone() {
            let temp: Vec<FullRecipe> = recipes.iter()
                .filter(|x| x.book_id.filter(|y| *y == lol_book.book_id.unwrap()).is_some())
                .filter(|x| x.primary_season == season.value_rofl() as i32)
                .map(|x| x.clone())
                .collect();
            if !temp.is_empty() {
                vals.push((lol_book.book_name.unwrap(), temp));
            }
        }

        for web_page in web_pages.clone() {
            let temp: Vec<FullRecipe> = recipes.iter()
                .filter(|x| x.book_id.is_none())
                .filter(|x| x.recipe_url.as_ref().filter(|y| y.contains(web_page.as_str())).is_some())
                .filter(|x| x.primary_season == season.value_rofl() as i32)
                .map(|x| x.clone())
                .collect();
            if !temp.is_empty() {
                vals.push((web_page, temp));
            }
        }


        let here_temp: Vec<FullRecipe> = recipes.iter()
            .filter(|x| x.book_id.is_none())
            .filter(|x| x.recipe_url.is_none())
            .filter(|x| x.primary_season == season.value_rofl() as i32)
            .map(|x| x.clone())
            .collect();
        if !here_temp.is_empty() {
            vals.push(("Here".to_string(), here_temp))
        }

        recipes_by_season_and_source.push((season, vals));
    }

    let content = CourseTemplate {
        course_name: asdf.course_name.as_ref().unwrap().as_str(),
        seasons: ESeason::get_seasons(),
        books: &books,
        courses: course_refs,
        title: cur_name,
        tried: tried_ids,
        logged_in: maybe_user_id.is_some(),
        recipes_to_ingredients: recipes_to_ingredients,
        user_id: maybe_user_id,
        recipes_by_season_and_source,
    }.get();

    return Html(content);
}


async fn recipe_form(session: ReadableSession, prefill: Query<RecipePrefill>) -> Response {
    let maybe_user_id = session.get::<i32>("user_id");
    if maybe_user_id.is_none() {
        return Redirect::to("/login").into_response();
    }
    let con = &mut database::establish_connection();

    use recipemanagement::schema::book::dsl::*;

    let books: Vec<QBook> = book.load::<QBook>(con).unwrap();
    use recipemanagement::schema::course::dsl::*;

    let courses: Vec<QCourse> = course.load::<QCourse>(con).unwrap();
    let course_refs: &Vec<QCourse> = &courses;
    use recipemanagement::schema::recipe::dsl::*;
    let newest_recipe: FullRecipe = recipe.order(recipemanagement::schema::recipe::recipe_id.desc()).first::<FullRecipe>(con)
        .unwrap();


    return Html(RecipeForm {
        seasons: ESeason::get_seasons(),
        books: &books,
        courses: course_refs,
        prefill: prefill.0,
        title: "Add Recipe",
        newest: newest_recipe.recipe_name.unwrap(),
        user_id: maybe_user_id,
    }
        .get()
    )
        .into_response();
}

#[derive(Deserialize)]
struct PostRecipe {
    course: i32,
    book: Option<String>,
    season: i32,
    name: String,
    url: Option<String>,
    page: Option<String>,
    recipe_url: Option<String>,
    recipe_text: Option<String>,
    ingredients: Option<String>,

}

async fn post_recipe(session: ReadableSession, Form(form): Form<PostRecipe>) -> Response {
    let maybe_user_id = session.get::<i32>("user_id");
    if maybe_user_id.is_none() {
        return Redirect::to("/login").into_response();
    }
    let con = &mut database::establish_connection();
    trace!("Adding {} with ingrdients: {}", form.name.clone(), form.ingredients.clone().unwrap_or("-".to_string()));

    let book_id = form.book.map(|x| x.parse::<i32>()).and_then(|x| x.ok()).filter(|x| *x > 0);
    let page = form.page.map(|x| x.parse::<i32>()).and_then(|x| x.ok());
    let recipe_struct = InsertRecipeWithUrl { recipe_id: None, recipe_name: form.name, primary_season: form.season, course_id: form.course, book_id: book_id, page: page, recipe_url: form.recipe_url };
    con.transaction::<_, Error, _>(|x| {
        use recipemanagement::schema::recipe;
        diesel::insert_into(recipemanagement::schema::recipe::table)
            .values(vec![recipe_struct])
            .execute(x)
            .unwrap();

        use recipemanagement::schema::recipe::dsl::*;
        let cur_recipe_id: i32 = recipe.order(recipe_id.desc()).first::<FullRecipe>(x)
            .unwrap()
            .recipe_id
            .unwrap();

        if form.recipe_text.as_ref().filter(|x| !(x.trim()).is_empty()).is_some() {
            let edit_recipe_text = InsertRecipeText { recipe_id: cur_recipe_id, content: form.recipe_text.unwrap() };
            use recipemanagement::schema::recipe_text::dsl::*;
            diesel::replace_into(recipe_text)
                .values(vec![edit_recipe_text])
                .execute(x)
                .unwrap();
        }

        let ingredient_string = form.ingredients.unwrap_or("".to_string());
        let ingredients_insert_vals: Vec<String> = ingredient_string.clone()
            .split("\n")
            .into_iter()
            .map(|y| format!("{}", y.trim()))
            .collect();
        let ingredients_relation_insert_vals: String = ingredient_string.clone()
            .split("\n")
            .into_iter()
            .map(|y| format!("'{}'", y.trim()))
            .join(",");
        for val in ingredients_insert_vals.iter().filter(|x| !(x.trim().is_empty())) {
            diesel::sql_query("INSERT OR IGNORE into  ingredient(name) values (?);")
                .bind::<Text, _>(val.clone())
                .execute(x);
            let res = diesel::sql_query("INSERT OR IGNORE INTO recipe_ingredient(recipe_id, ingredient_id)  SELECT ?, id FROM ingredient where lower(name)=?;")
                .bind::<Integer, _>(cur_recipe_id)
                .bind::<Text, _>(val.clone())
                .execute(x);
        }
        return Ok(());
    }
    ).unwrap();
    let url = format!("/recipe/add?season={}&course={}&book={}", form.season, form.course, book_id.unwrap_or(0));


    return Redirect::to(url.as_str()).into_response();
}


#[derive(Deserialize)]
struct PostBook {
    booktext: String,
}

async fn book_form(session: ReadableSession) -> Response {
    let maybe_user_id = session.get::<i32>("user_id");
    if maybe_user_id.is_none() {
        return Redirect::to("/login").into_response();
    }
    let con = &mut database::establish_connection();

    let courses: Vec<QCourse> = course.load::<QCourse>(con).unwrap();
    let course_refs: &Vec<QCourse> = &courses;


    return Html(BookForm {
        courses: course_refs,
        title: "Add book",
        user_id: maybe_user_id,
    }.get()).into_response();
}

async fn post_book(session: ReadableSession, Form(form): Form<PostBook>) -> Redirect {
    let maybe_user_id = session.get::<i32>("user_id");
    if maybe_user_id.is_none() {
        return Redirect::to("/login");
    }
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



async fn search_form(session: ReadableSession) -> Response {
    let maybe_user_id = session.get::<i32>("user_id");
    if maybe_user_id.is_none() {
        return Redirect::to("/login").into_response();
    }

    let con = &mut database::establish_connection();

    use recipemanagement::schema::book::dsl::*;

    let books: Vec<QBook> = book.load::<QBook>(con).unwrap();
    use recipemanagement::schema::course::dsl::*;

    let courses: Vec<QCourse> = course.load::<QCourse>(con).unwrap();
    let course_refs: &Vec<QCourse> = &courses;


    return Html(SearchForm {
        seasons: ESeason::get_seasons(),
        books: &books,
        courses: course_refs,
        recipes: None,
        title: "Search",
        recipes_to_ingredients: Default::default(),
        user_id: maybe_user_id,
    }.get()).into_response();
}

async fn search_result(session: ReadableSession, Form(form): Form<SearchRecipe>) -> Response {
    let maybe_user_id = session.get::<i32>("user_id");
    if maybe_user_id.is_none() {
        return Redirect::to("/login").into_response();
    }

    let con = &mut database::establish_connection();



    use recipemanagement::schema::book::dsl::*;

    let books: Vec<QBook> = book.load::<QBook>(con).unwrap();
    use recipemanagement::schema::course::dsl::*;

    let courses: Vec<QCourse> = course.load::<QCourse>(con).unwrap();
    let course_refs: &Vec<QCourse> = &courses;


    use recipemanagement::schema::book::dsl::*;

    let books: Vec<QBook> = book.load::<QBook>(con).unwrap();

    let query_string = build_search_query(form, maybe_user_id.unwrap());
    let recipes = sql_query(query_string)
        .load::<FullRecipe>(con)
        .ok().unwrap_or(vec![]);

    use recipemanagement::schema::ingredient::dsl::*;
    let id_to_ingredients: HashMap<i32, String> = ingredient.load::<Ingredient>(con)
        .unwrap()
        .iter()
        .map(|x| (x.clone().id.unwrap(), x.name.clone().unwrap()))
        .collect();

    use recipemanagement::schema::recipe_ingredient::dsl::*;
    let recipes_to_ingredients = recipe_ingredient.load::<RecipeIngredient>(con)
        .unwrap()
        .iter()
        .map(|x| (x.recipe_id, id_to_ingredients.get(&x.ingredient_id)))
        .filter(|x| x.clone().1.is_some())
        .map(|x| (x.clone().0, x.clone().1.unwrap().clone()))
        .into_group_map();


    return Html(SearchForm {
        seasons: ESeason::get_seasons(),
        books: &books,
        courses: course_refs,
        recipes: Some(recipes),
        title: "Search",
        recipes_to_ingredients,
        user_id: maybe_user_id,
    }
        .get()).into_response();
}

async fn login_page(session: ReadableSession) -> Html<String> {
    let con = &mut database::establish_connection();
    let maybe_user_id = session.get::<i32>("user_id");

    let courses: Vec<QCourse> = course.load::<QCourse>(con).unwrap();
    return Html(LoginPage {
        courses: &courses,
        title: "Login",
        user_id: maybe_user_id,
    }.get());
}

#[derive(Deserialize)]
struct Login {
    email: String,
    password: String,
}

async fn my_login(mut session: WritableSession, Form(form): Form<Login>) -> Redirect { //

    let con = &mut database::establish_connection();

    let sql_string = format!("SELECT * FROM user WHERE email='{}'", form.email);
    let temp_user = sql_query(sql_string)
        .load::<User>(con)
        .unwrap();
    let maybe_user = temp_user.first();
    if maybe_user.as_ref().is_none() {
        return Redirect::to("/search");
    }

    let parsed_hash = PasswordHash::new(maybe_user.unwrap().pw_hash.as_str()).ok().unwrap();
    let verif = Argon2::default().verify_password(form.password.as_bytes(), &parsed_hash);

    if verif.is_ok() {
        session.insert("user_id", maybe_user.as_ref().unwrap().id.unwrap());
    }


    return Redirect::to("/");
}

async fn edit_recipe_form(session: ReadableSession, Path(path): Path<i32>) -> Response {
    let maybe_user_id = session.get::<i32>("user_id");
    if maybe_user_id.is_none() {
        return Redirect::to("/login").into_response();
    }
    use recipemanagement::schema::recipe::dsl::*;
    let con = &mut database::establish_connection();

    let query = recipe
        .filter(recipemanagement::schema::recipe::recipe_id.eq(path))
        .load::<FullRecipe>(con)
        .unwrap();

    let das_recipe = query
        .first();

    if das_recipe.is_none() {
        return Html("404'd".to_string()).into_response();
    }
    let courses: Vec<QCourse> = course.load::<QCourse>(con).unwrap();
    use recipemanagement::schema::book::dsl::*;

    let books: Vec<QBook> = book.load::<QBook>(con).unwrap();


    let ingredient_query_sql = format!("
SELECT i.*
FROM recipe_ingredient INNER JOIN ingredient i on i.id = recipe_ingredient.ingredient_id
WHERE recipe_id={}", path);
    let ingredients = sql_query(ingredient_query_sql)
        .load::<Ingredient>(con)
        .unwrap();
    let ingredient_prefill = ingredients.iter().map(|x| x.name.as_ref().unwrap()).join("\n");
    let prefill_season = das_recipe.as_ref().unwrap().primary_season as usize;
    use recipemanagement::schema::recipe_text::dsl::*;
    let recipe_text_disp = recipe_text.filter(recipemanagement::schema::recipe_text::recipe_id.eq(path))
        .load::<RecipeText>(con)
        .unwrap().first().map(|x| x.content.clone()).unwrap_or("".to_string());


    return Html(RecipeEditForm {
        courses: &courses,
        recipe: das_recipe.as_ref().unwrap(),
        ingredients: ingredient_prefill,
        title: "Edit recipe",
        books: &books,
        seasons: ESeason::get_seasons(),
        prefill_season: prefill_season,
        recipe_text: recipe_text_disp,
        user_id: maybe_user_id,
    }.get())
        .into_response();
}

#[derive(Deserialize)]
struct PutRecipe {
    name: String,
    book: Option<i32>,
    course: i32,
    season: i32,
    ingredients: Option<String>,
    page: Option<String>,
    url: Option<String>,
    recipe_text: Option<String>
}

async fn put_recipe(session: ReadableSession, Path(path): Path<i32>, Form(form): Form<PutRecipe>) -> Redirect {
    let maybe_user_id = session.get::<i32>("user_id");
    if maybe_user_id.is_none() {
        return Redirect::to("/login");
    }

    let con = &mut database::establish_connection();

    let page_res = form.page.map(|x| x.parse::<i32>()).and_then(|x| x.ok());


    let transaction_res = con.transaction::<_, Error, _>(|x| {
        use recipemanagement::schema::recipe::dsl::*;

        let old_recipe_query = recipe.filter(recipe_id.eq(path))
            .load::<FullRecipe>(x)
            .unwrap();
        let old_recipe = old_recipe_query.first().unwrap();
        let update_url = form.url.filter(|x| !(x.trim().is_empty()));


        let edit_recipe = FullInsertRecipe {
            recipe_id: Some(path),
            recipe_url: update_url,
            recipe_name: Some(form.name),
            primary_season: form.season,
            course_id: form.course,
            created_at: old_recipe.created_at,
            page: page_res,
            book_id: form.book,
        };

        diesel::replace_into(recipe)
            .values(&vec![edit_recipe])
            .execute(x);
        if form.ingredients.is_some() {
            use recipemanagement::schema::ingredient::dsl::*;
            let ingredient_names: Vec<String> = form.ingredients.unwrap().trim()
                .split("\n")
                .map(|x| x.trim())
                .map(|x| x.to_string())
                .collect();


            let existing_ingredients = ingredient
                .load::<Ingredient>(x)
                .unwrap();

            let ingredient_to_id: HashMap<String, i32> = existing_ingredients.iter().map(|x| (x.name.as_ref().clone().unwrap().to_string(), *x.id.as_ref().unwrap()))
                .collect();
            let id_to_ingredient: HashMap<i32, String> = existing_ingredients.iter().map(|x| (*x.id.as_ref().unwrap(), x.name.as_ref().unwrap().to_string()))
                .collect();
            let existing_names: HashSet<String, RandomState> = HashSet::from_iter(existing_ingredients.iter().map(|x| x.name.as_ref().unwrap().to_string()));
            let update_names = HashSet::from_iter(ingredient_names.iter().map(|x| x.to_string()));

            use recipemanagement::schema::recipe_text::dsl::*;
            let edit_recipe_text = InsertRecipeText { recipe_id: path, content: form.recipe_text.unwrap_or("".to_string()) };
            diesel::replace_into(recipe_text)
                .values(vec![edit_recipe_text])
                .execute(x)
                .unwrap();



            use recipemanagement::schema::recipe_ingredient::dsl::*;

            let assigned_ingredients = recipe_ingredient.filter(recipemanagement::schema::recipe_ingredient::recipe_id.eq(path))
                .load::<RecipeIngredient>(x)
                .unwrap();

            let assigned_names: HashSet<String, RandomState> = HashSet::from_iter((assigned_ingredients.iter().map(|x| id_to_ingredient.get(&x.ingredient_id).unwrap().to_string())));

            let to_connect = update_names.intersection(&existing_names)
                .into_iter()
                .map(|x| x.to_string())
                .collect::<HashSet<String, RandomState>>()
                .difference(&assigned_names)
                .map(|x| x.to_string())
                .collect::<HashSet<String, RandomState>>();
            let to_delete: HashSet<String> = assigned_names.difference(&update_names).into_iter().map(|x| (x.clone())).collect();
            let to_insert: HashSet<String> = update_names.difference(&existing_names).into_iter().map(|x| (x.clone())).collect();

            let connect_obs: Vec<InsertRecipeIngredient> = to_connect.into_iter()
                .map(|x| ingredient_to_id.get(x.as_str()).unwrap())
                .map(|x| InsertRecipeIngredient { recipe_id: path, ingredient_id: *x })
                .collect();
            diesel::insert_into(recipe_ingredient)
                .values(connect_obs)
                .execute(x)
                .unwrap();


            let delete_ids: Vec<i32> = to_delete.into_iter().map(|x| *(ingredient_to_id.get(x.as_str()).unwrap()))
                .collect();
            diesel::delete(recipe_ingredient.filter(recipemanagement::schema::recipe_ingredient::recipe_id.eq(path)).filter(ingredient_id.eq_any(&delete_ids)))
                .execute(x).unwrap();

            let start_id: i32 = ingredient.select(max(id))
                .first::<Option<i32>>(x)
                .unwrap()
                .map(|x| x + 1)
                .unwrap_or(1);


            let new_ingredients: Vec<InsertIngredient> = to_insert.iter().enumerate()
                .map(|(i, x)| InsertIngredient { id: Some(i as i32 + start_id), name: x.to_string() })
                .collect();
            let new_refs: Vec<InsertRecipeIngredient> = to_insert.iter()
                .enumerate()
                .map(|(i, x)| InsertRecipeIngredient { recipe_id: path, ingredient_id: i as i32 + start_id })
                .collect();

            diesel::insert_into(ingredient)
                .values(new_ingredients)
                .execute(x)
                .unwrap();
            diesel::insert_into(recipe_ingredient)
                .values(new_refs)
                .execute(x)
                .unwrap();
        }


        Ok(())
    }
    );
    return Redirect::to(format!("/recipe/detail/{}", path).as_str())
}

async fn toggle_tried(session: ReadableSession, Path(path): Path<i32>) -> StatusCode {
    let maybe_user_id = session.get::<i32>("user_id");
    if maybe_user_id.is_none() {
        return StatusCode::UNAUTHORIZED;
    }
    let connection = &mut database::establish_connection();
    let transaction_res = connection.transaction::<_, Error, _>(|con| {
        use recipemanagement::schema::tried::dsl::*;
        let already_exists = select(
            exists(
                tried.filter(user_id.eq(maybe_user_id.unwrap()))
                    .filter(recipe_id.eq(path))
            )
        ).get_result::<bool>(con)
            .unwrap();
        if !already_exists {
            diesel::insert_into(tried)
                .values((&recipe_id.eq(path), &user_id.eq(maybe_user_id.unwrap())))
                .execute(con).unwrap();
        } else {
            diesel::delete(tried.filter(user_id.eq(maybe_user_id.unwrap()))
                .filter(recipe_id.eq(path))).execute(con).unwrap();
        }


        return Ok(());
    });


    return StatusCode::OK;
}

fn test(id_to_ingredient: HashMap<i32, String>) {
    id_to_ingredient.get(&15).unwrap();
}

pub struct RecipeEditQuery {}


fn query_for_recipe_detail<'a, 'b>(con: &mut LoggingConnection<SqliteConnection>, path: i32, cur_user_id: i32) -> Result<Option<RecipeDetailQuery>, Error> {
    use recipemanagement::schema::recipe::dsl::*;

    let query = recipe
        .filter(recipemanagement::schema::recipe::recipe_id.eq(path))
        .load::<FullRecipe>(con)
        .unwrap();

    let das_recipe = query
        .first();
    if das_recipe.is_none() {
        return Ok::<Option<RecipeDetailQuery>, Error>(None);
    }

    use recipemanagement::schema::book::dsl::*;

    let disp_book: Option<String> = book.filter(recipemanagement::schema::book::dsl::book_id.eq(das_recipe.unwrap().book_id)).load::<QBook>(con).unwrap()
        .first()
        .map(|x| x.book_name.as_ref().unwrap().clone());

    let courses: Vec<QCourse> = course.load::<QCourse>(con).unwrap();
    let res_recipe = das_recipe.unwrap();

    let course_name: String = courses.iter()
        .filter(|x| x.course_id.is_some() && x.course_id.unwrap() == res_recipe.course_id)
        .map(|x| x.course_name.clone())
        .next()
        .unwrap()
        .unwrap();

    let ingredient_query_sql = format!("
SELECT i.*
FROM recipe_ingredient INNER JOIN ingredient i on i.id = recipe_ingredient.ingredient_id
WHERE recipe_id={}", path);
    let ingredients: Vec<String> = sql_query(ingredient_query_sql)
        .load::<Ingredient>(con)
        .unwrap()
        .iter()
        .map(|x| x.name.as_ref().unwrap().clone())
        .collect();

    use recipemanagement::schema::tried::dsl::*;
    let already_exists = select(
        exists(
            tried.filter(recipemanagement::schema::tried::user_id.eq(cur_user_id))
                .filter(recipemanagement::schema::tried::recipe_id.eq(path))
        )
    ).get_result::<bool>(con).unwrap();

    use recipemanagement::schema::recipe_comment::dsl::*;

    let comments = recipe_comment.filter(recipemanagement::schema::recipe_comment::recipe_id.eq(path))
        .load::<Comment>(con)
        .unwrap();
    use recipemanagement::schema::recipe_text::dsl::*;

    let recipe_text_disp = recipe_text.filter(recipemanagement::schema::recipe_text::recipe_id.eq(path))
        .load::<RecipeText>(con)
        .unwrap().first().map(|x| x.content.clone()).unwrap_or("".to_string());


    return Ok(Some(RecipeDetailQuery {
        courses: courses,
        course: course_name,
        recipe: res_recipe.clone(),
        ingredients: ingredients,
        title: res_recipe.recipe_name.clone().unwrap(),
        book_name: disp_book,
        season: ESeason::get_by_db_id(res_recipe.primary_season),
        tried: already_exists,
        comments: comments,
        recipe_text: recipe_text_disp,
    }));
}

struct RecipeDetailQuery {
    courses: Vec<QCourse>,
    course: String,
    recipe: FullRecipe,
    ingredients: Vec<String>,
    title: String,
    book_name: Option<String>,
    season: ESeason,
    tried: bool,
    comments: Vec<Comment>,
    recipe_text: String

}


async fn recipe_detail(session: ReadableSession, Path(path): Path<i32>) -> Response {
    let maybe_user_id = session.get::<i32>("user_id");
    if maybe_user_id.is_none() {
        return Redirect::to("/login").into_response();
    }
    let con = &mut database::establish_connection();
    let res = con.transaction(|x| query_for_recipe_detail(x, path, maybe_user_id.unwrap()));
    return Html(res.ok()
        .unwrap()
        .map(|x| RecipeDetail {
            courses: &x.courses,
            course: x.course.as_str(),
            recipe: &x.recipe,
            ingredients: x.ingredients,
            title: x.title.as_str(),
            book_name: &x.book_name,
            season: x.season,
            tried: x.tried,
            comments: x.comments,
            recipe_text: x.recipe_text,
            user_id: maybe_user_id,
        }.get())
        .unwrap_or("404".to_string())
    ).into_response();
}

#[derive(Deserialize)]
struct PostComment {
    comment: String,
}

async fn post_comment(session: ReadableSession, Path(path): Path<i32>, Form(form): Form<PostComment>) -> Response {
    let maybe_user_id = session.get::<i32>("user_id");
    if maybe_user_id.is_none() {
        return Redirect::to("/login").into_response();
    }
    let con = &mut database::establish_connection();


    if !form.comment.trim().is_empty() {
        let cur_user_id = maybe_user_id.unwrap();
        use recipemanagement::schema::recipe_comment::dsl::*;
        let insert_comment = InsertComment {
            user_id: cur_user_id,
            recipe_id: path,
            content: form.comment.trim().to_string(),
        };

        diesel::insert_into(recipemanagement::schema::recipe_comment::table)
            .values(vec![insert_comment])
            .execute(con)
            .unwrap();
    }


    return Redirect::to(format!("/recipe/detail/{}", path).as_str()).into_response();
}


fn query_course() {}



