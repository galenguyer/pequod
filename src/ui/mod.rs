use axum::{extract::Path, http::{StatusCode, HeaderMap}, response::IntoResponse, Extension};
use tera::{Context, Tera};

pub async fn index(Extension(tera): Extension<Tera>) -> impl IntoResponse {
    let repos: Vec<String> = crate::db::sqlite::repositories::list()
        .await
        .unwrap()
        .into_iter()
        .map(|r| r.name)
        .collect();

    let categories = repos
        .iter()
        .filter(|r| r.contains("/"))
        .map(|r| r.split("/").next().unwrap().to_string())
        .collect::<Vec<String>>();
    let repos = repos
        .iter()
        .filter(|r| !r.contains("/"))
        .map(|r| r.to_string())
        .collect::<Vec<String>>();

    let mut context = Context::new();
    context.insert("categories", &categories);
    context.insert("repos", &repos);
    (
        StatusCode::OK,
        [("Content-Type", "text/html; charset=utf-8")],
        tera.render("index.html", &context).unwrap(),
    )
}

pub async fn repo(Path(name): Path<String>, Extension(tera): Extension<Tera>, headers: HeaderMap) -> impl IntoResponse {
    let repos: Vec<String> = crate::db::sqlite::repositories::list()
        .await
        .unwrap()
        .into_iter()
        .map(|r| r.name)
        .filter(|r| r.contains("/") && r.starts_with(&name))
        .map(|r| r.replace(&name, "").trim_start_matches('/').to_string())
        .collect();

    let categories = repos
        .iter()
        .filter(|r| r.contains("/"))
        .map(|r| r.split("/").next().unwrap().to_string())
        .collect::<Vec<String>>();
    let repos = repos
        .iter()
        .filter(|r| !r.contains("/"))
        .map(|r| r.to_string())
        .collect::<Vec<String>>();

    let tags = crate::db::sqlite::tags::list(&name).await.unwrap();


    let mut context = Context::new();
    context.insert("name", &name.trim_matches('/'));
    context.insert("categories", &categories);
    context.insert("repos", &repos);
    context.insert("tags", &tags);
    context.insert("host", headers.get("host").unwrap().to_str().unwrap());

    (
        StatusCode::OK,
        [("Content-Type", "text/html; charset=utf-8")],
        tera.render("repo.html", &context).unwrap(),
    )
}
