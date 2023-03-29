use std::collections::{HashSet, HashMap};

use axum::{extract::Path, http::{StatusCode, HeaderMap}, response::IntoResponse, Extension};
use tera::{Context, Tera};
use bytesize::ByteSize;

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
        .collect::<HashSet<String>>();
    let repos = repos
        .iter()
        .filter(|r| !r.contains("/"))
        .map(|r| r.to_string())
        .collect::<HashSet<String>>();

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
        .collect::<HashSet<String>>();
    let repos = repos
        .iter()
        .filter(|r| !r.contains("/"))
        .map(|r| r.to_string())
        .collect::<HashSet<String>>();

    let tags = crate::db::sqlite::tags::list(&name).await.unwrap();
    let mut sizes = HashMap::new();
    for tag in tags.iter() {
        let size = crate::db::sqlite::tags::get_size(&tag.manifest).await.unwrap_or_default();
        let size = ByteSize::b(size as u64).to_string_as(true);
        sizes.insert(tag.name.clone(), size);
    }

    let mut context = Context::new();
    context.insert("name", &name.trim_matches('/'));
    context.insert("categories", &categories);
    context.insert("repos", &repos);
    context.insert("tags", &tags);
    context.insert("sizes", &sizes);
    context.insert("host", headers.get("host").unwrap().to_str().unwrap());

    (
        StatusCode::OK,
        [("Content-Type", "text/html; charset=utf-8")],
        tera.render("repo.html", &context).unwrap(),
    )
}

pub async fn admin(Extension(tera): Extension<Tera>) -> impl IntoResponse {
    let size = std::fs::metadata("registry.db").unwrap().len();
    let size = ByteSize::b(size).to_string_as(true);

    let mut context = Context::new();
    context.insert("size", &size);
    (
        StatusCode::OK,
        [("Content-Type", "text/html; charset=utf-8")],
        tera.render("admin.html", &context).unwrap(),
    )
}

pub async fn cleanup(Extension(tera): Extension<Tera>) -> impl IntoResponse {
    let old_size = std::fs::metadata("registry.db").unwrap().len();
    let old_size = ByteSize::b(old_size).to_string_as(true);

    crate::db::sqlite::cleanup().await.unwrap();

    let size = std::fs::metadata("registry.db").unwrap().len();
    let size = ByteSize::b(size).to_string_as(true);

    let mut context = Context::new();
    context.insert("size", &size);
    context.insert("old_size", &old_size);
    (
        StatusCode::OK,
        [("Content-Type", "text/html; charset=utf-8")],
        tera.render("admin.html", &context).unwrap(),
    )
}
