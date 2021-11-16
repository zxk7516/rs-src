use std::{
    convert::Infallible,
    net::SocketAddr,
};

use axum::{
    routing::{
        get,
        post,
    },
    Json,
    Router,
};
use serde::{
    Deserialize,
    Serialize,
};
use tokio::runtime::Builder;

fn main() {
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();
    rt.block_on(async {
        if std::env::var_os("RUST_LOG").is_none() {
            std::env::set_var("RUST_LOG", "example_jwt=debug");
        }
        tracing_subscriber::fmt::init();

        let app = Router::new()
            .route("/protected", get(protected))
            .route("/authorize", post(authorize));
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        tracing::debug!("listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });
}

#[derive(Debug, Deserialize)]
struct AuthPayload {
    username: String,
    password: String,
}
#[derive(Debug, Serialize)]
struct AuthBody {
    auth_type: String,
    token: String,
    ttl: u64,
}
enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

async fn protected() -> Result<String, AuthError> {
    Ok("Hello world".to_string())
}
async fn authorize(Json(payload): Json<AuthPayload>) -> Result<AuthPayload, AuthError> {
    todo!()
}
