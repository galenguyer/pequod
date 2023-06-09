use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use crate::db;

pub async fn tags(Path(name): Path<String>) -> impl IntoResponse {
    let tags = db::tags::list(&name).await.unwrap();

    (
        StatusCode::OK,
        Json(json!({
            "name": name.clone(),
            "tags": tags.into_iter().map(|t| t.name).collect::<Vec<String>>()
        })),
    )
}
