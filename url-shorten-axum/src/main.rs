use std::error::Error;
use std::net::SocketAddr;
use std::time::Duration;

pub mod dto;
pub mod handlers;
pub mod models;

use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    response::IntoResponse,
    routing::{
        get,
        post,
    },
    AddExtensionLayer,
    BoxError,
    Router,
};

use handlers::{
    create_short,
    delete_short,
    get_short,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[cfg(feature = "mysql")]
pub type DBPool = sqlx::mysql::MySqlPool;
#[cfg(feature = "mysql")]
pub type QueryResult = sqlx::mysql::MySqlQueryResult;
#[cfg(feature = "mysql")]
pub type DBOptions = sqlx::mysql::MySqlPoolOptions;

#[cfg(feature = "postgres")]
pub type DBPool = sqlx::postgres::PgPool;
#[cfg(feature = "postgres")]
pub type QueryResult = sqlx::postgres::PgQueryResult;
#[cfg(feature = "postgres")]
pub type DBOptions = sqlx::postgres::PgPoolOptions;

async fn root() -> &'static str {
    "Hello world!"
}

#[cfg(feature = "mysql")]
const DB_URL: &'static str = "mysql://root:root@127.0.0.1/short_links";
#[cfg(feature = "postgres")]
const DB_URL: &'static str = "postgres://127.0.0.1/short_links?useSSL=false";

async fn handle_error<T>(_error: T) -> axum::http::StatusCode {
    axum::http::StatusCode::INTERNAL_SERVER_ERROR
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("before start");
    let url = std::env::var("URLSHORTER_DB");
    let pool: DBPool = DBOptions::new()
        .connect(match url {
            Ok(ref u) => &u,
            _ => DB_URL,
        })
        .await?;

    println!("db pool created");
    let middleware_stack = ServiceBuilder::new()
        .layer(AddExtensionLayer::new(pool))
        .layer(HandleErrorLayer::new(handle_error))
        .layer(TraceLayer::new_for_http())
        .timeout(Duration::from_secs(10));

    let app = Router::new()
        .route("/", get(root))
        .route("/api/create_short", post(create_short))
        .route("/api/delete_short", post(delete_short))
        .route("/:id", get(get_short))
        .layer(middleware_stack);
    println!("app initialized");

    let addr = SocketAddr::from(([0, 0, 0, 0], 3412));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    println!("server running...");

    Ok(())
}
