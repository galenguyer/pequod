use axum::{routing, Router};

pub mod api;
pub mod db;

#[tokio::main]
async fn main() {
    let router = Router::new().nest(
        "/v2",
        Router::new()
            .route("/", routing::get(api::base))
            .route("/_catalog", routing::get(api::catalog)),
    );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
