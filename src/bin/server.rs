use axum::{Router, routing::get};
use axum::extract::Path;
use diesel::prelude::*;
use itertools::Itertools;
use recipemanagement::*;
use recipemanagement::models::*;
use recipemanagement::templates::*;
use std::fmt::format;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Route all requests on "/" endpoint to anonymous handler.
    //
    // A handler is an async function which returns something that implements
    // `axum::response::IntoResponse`.

    // A closure or a function can be used as handler.

    let app = Router::new().route("/", get(handler))
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

async fn handler() -> String {
    let con = &mut database::establish_connection();
    use recipemanagement::schema::recipe::dsl::*;
    let res: Vec<FullRecipe> = recipe.limit(20).load::<FullRecipe>(con).unwrap();
    return res.iter().map(|x| x.recipe_name.clone().unwrap()).join("\n");
}

async fn handle_course(Path(path): Path<String>) -> String {
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
    use recipemanagement::schema::recipe::dsl::*;
    let recipes: Vec<FullRecipe> = recipe.filter(recipemanagement::schema::recipe::course_id.eq(res)).load::<FullRecipe>(con).unwrap();

    let out = recipes.iter().map(|x| x.recipe_name.clone().unwrap()).join("\n");
    let hello = HelloTemplate { name: "world" }; // instantiate your struct
    println!("{}", hello.get()); // then render it.

    return out;
}

fn query_course() {}

