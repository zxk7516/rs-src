use std::{
    convert::Infallible,
    sync::Arc,
};

use warp::Filter;

use crate::todo_rest::todo_filter;

mod security;
mod todo_rest;
const WEB_FOLDER: &str = "web-folder/";

#[tokio::main]
async fn main() {
    let db_pool = Arc::new(DbPool{});
    let hello_world = warp::path::end().map(|| "Hello world from root.");

    let hi = warp::path("hi").and(warp::get()).map(|| "Hello from Hi");
    let apis = hi.or(todo_filter(db_pool.clone()));

    // static content
    let content = warp::fs::dir(WEB_FOLDER);

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", WEB_FOLDER)));
    let static_site = content.or(index);

    let routes = apis.or(static_site);

    println!("start web server");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await
}

pub struct DbPool {}

pub fn with_db_pool(
    db_pool: Arc<DbPool>
) -> impl Filter<Extract = (Arc<DbPool>,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
