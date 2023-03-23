use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

pub async fn tags(Path(name): Path<String>) -> impl IntoResponse {
    dbg!(&name);
    let tags = crate::db::sqlite::tags::list(&name).await.unwrap();

    (
        StatusCode::OK,
        Json(json!({
            "name": name.clone(),
            "tags": tags
        })),
    )
}
