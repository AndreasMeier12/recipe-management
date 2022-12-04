use std::collections::HashMap;
use std::fmt::format;
use std::net::SocketAddr;
use std::ops::Deref;

use axum::{Router, routing::get};
use axum::{body::Body, response::{Html, Json}};
use axum::extract::Path;
use diesel::prelude::*;
use itertools::Itertools;

use recipemanagement::*;
use recipemanagement::models::*;
use recipemanagement::parsetypes::ESeason;
use recipemanagement::templates::*;

#[tokio::main]
async fn main() {
    // Route all requests on "/" endpoint to anonymous handler.
    //
    // A handler is an async function which returns something that implements
    // `axum::response::IntoResponse`.

    // A closure or a function can be used as handler.

    let app = Router::new().route("/", get(index_handler))
        .route("/course/:name", get(handle_course));
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

    let content = CourseTemplate { course_name: asdf.course_name.as_ref().unwrap().as_str(), seasons: ESeason::get_seasons(), recipes_per_book_season: recipes_per_book_season, books: &books, courses: course_refs }.get();

    return Html(content);
}

fn query_course() {}

