use axum::{routing, Router};

mod api;

#[tokio::main]
async fn main() {
    let router = Router::new().nest("/v2", Router::new().route("/", routing::get(api::base)));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
