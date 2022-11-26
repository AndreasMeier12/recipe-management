use std::net::SocketAddr;

use axum::{Router, routing::get};
use diesel::prelude::*;
use itertools::Itertools;

use recipemanagement::*;
use recipemanagement::models::*;

#[tokio::main]
async fn main() {
    // Route all requests on "/" endpoint to anonymous handler.
    //
    // A handler is an async function which returns something that implements
    // `axum::response::IntoResponse`.

    // A closure or a function can be used as handler.

    let app = Router::new().route("/", get(handler));
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