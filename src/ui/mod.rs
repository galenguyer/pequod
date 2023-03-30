use std::collections::{HashMap, HashSet};

use axum::{
    extract::Path,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Extension,
};
use bytesize::ByteSize;
use chrono::{DateTime, Utc};
use serde::Serialize;
use tera::{Context, Tera};

use crate::db;

pub async fn index(Extension(tera): Extension<Tera>) -> impl IntoResponse {
    let repos: Vec<String> = db::repositories::list()
        .await
        .unwrap()
        .into_iter()
        .map(|r| r.name)
        .collect();

    let categories = {
        let mut c = repos
            .iter()
            .filter(|r| r.contains("/"))
            .map(|r| r.split("/").next().unwrap().to_string())
            .collect::<Vec<String>>();
        c.dedup();
        c.sort();
        c
    };

    let repos = {
        let mut r = repos
            .iter()
            .filter(|r| !r.contains("/"))
            .map(|r| r.to_string())
            .collect::<Vec<String>>();
        r.dedup();
        r.sort();
        r
    };

    let mut context = Context::new();
    context.insert("categories", &categories);
    context.insert("repos", &repos);
    (
        StatusCode::OK,
        [("Content-Type", "text/html; charset=utf-8")],
        tera.render("index.html", &context).unwrap(),
    )
}

#[derive(Debug, Serialize)]
struct TagGrouping {
    tags: Vec<String>,
    size: String,
    manifest: String,
    updated: DateTime<Utc>,
}
pub async fn repo(
    Path(name): Path<String>,
    Extension(tera): Extension<Tera>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let repos: Vec<String> = db::repositories::list()
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

    let tags = db::tags::list(&name).await.unwrap();
    let mut groupings: HashMap<String, TagGrouping> = HashMap::new();
    for tag in tags {
        match groupings.get_mut(&tag.manifest) {
            Some(group) => {
                group.tags.insert(0, tag.name.clone());
                group.tags.sort_by(|a, b| {
                    if a == "latest" {
                        std::cmp::Ordering::Less
                    } else if b == "latest" {
                        std::cmp::Ordering::Greater
                    } else {
                        a.cmp(b)
                    }
                });
                group.tags.dedup();
            }
            None => {
                groupings.insert(
                    tag.manifest.clone(),
                    TagGrouping {
                        tags: vec![tag.name.clone()],
                        size: {
                            let size = db::tags::get_size(&tag.manifest)
                                .await
                                .unwrap_or_default();
                            ByteSize::b(size as u64).to_string_as(true)
                        },
                        manifest: tag.manifest.clone(),
                        updated: tag.updated,
                    },
                );
            }
        }
    }
    let mut groupings = groupings.values().collect::<Vec<&TagGrouping>>();
    groupings.sort_by(|a, b| match a.updated.cmp(&b.updated) {
        std::cmp::Ordering::Equal => {
            if a.tags.contains(&"latest".to_string()) {
                std::cmp::Ordering::Less
            } else if b.tags.contains(&"latest".to_string()) {
                std::cmp::Ordering::Greater
            } else {
                a.tags[0].cmp(&b.tags[0])
            }
        }
        other => other,
    });

    let mut context = Context::new();
    context.insert("name", &name.trim_matches('/'));
    context.insert("categories", &categories);
    context.insert("repos", &repos);
    context.insert("groupings", &groupings);
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

    db::cleanup().await.unwrap();

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
