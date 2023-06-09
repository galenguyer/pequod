use axum::extract::DefaultBodyLimit;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::{routing, Extension, Router, ServiceExt};
use lazy_static::lazy_static;
use regex::Regex;
use tower::Layer;

pub mod api;
pub mod db;
pub mod ui;

lazy_static! {
    static ref URI_NAME_REGEX: Regex =
        Regex::new(r"/v2/(?P<name>[\w/]+)/(?P<resource>(tags|manifests|blobs))/").unwrap();
    static ref DIGEST_REGEX: Regex =
        Regex::new(r"^(?P<algorithm>[A-Za-z0-9_+.-]+):(?P<hex>[A-Fa-f0-9]+)$").unwrap();
}

#[async_backtrace::framed]
async fn rewrite_request_uri<B>(mut req: Request<B>, next: Next<B>) -> Response {
    let captures = match URI_NAME_REGEX.captures(req.uri().path()) {
        Some(captures) => captures,
        None => return next.run(req).await,
    };

    let old_name = captures.name("name").unwrap().as_str();
    let new_name = old_name.to_string().replace('/', "%2F");

    *req.uri_mut() = req
        .uri()
        .to_string()
        .replace(old_name, &new_name)
        .parse()
        .unwrap();

    next.run(req).await
}

#[tokio::main]
#[async_backtrace::framed]
async fn main() {
    dotenvy::dotenv().ok();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();

    let tera = match tera::Tera::new("src/ui/templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    for t in tera.get_template_names() {
        tracing::info!("loaded template {}", t);
    }

    let rewriter = axum::middleware::from_fn(rewrite_request_uri);
    let router = Router::new()
        .route("/", routing::get(ui::index))
        .route("/admin", routing::get(ui::admin))
        .route("/admin/cleanup", routing::post(ui::cleanup))
        .route("/*name", routing::get(ui::repo))
        .nest(
            "/v2",
            Router::new()
                .route("/", routing::get(api::base))
                .route("/_catalog", routing::get(api::catalog))
                .route("/:name/tags/list", routing::get(api::tags))
                .route(
                    "/:name/manifests/:reference",
                    routing::get(api::manifests::get)
                        .put(api::manifests::put)
                        .delete(api::manifests::delete),
                )
                .route(
                    "/:name/blobs/uploads/",
                    routing::post(api::blob::post_uploads),
                )
                .route(
                    "/:name/blobs/:digest",
                    routing::get(api::blob::get_blob)
                        .head(api::blob::head_blob)
                        .delete(api::blob::delete),
                )
                .route(
                    "/:name/blobs/uploads/:uuid",
                    routing::patch(api::blob::patch_uploads)
                        .put(api::blob::finish_uploads)
                        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024)),
                ),
        )
        .layer(Extension(tera));

    let app = rewriter.layer(router);

    async_backtrace::frame!(
        axum::Server::bind(&"0.0.0.0:5000".parse().unwrap()).serve(app.into_make_service())
    )
    .await
    .unwrap();
}

#[cfg(test)]
mod test {
    use super::URI_NAME_REGEX;

    #[test]
    fn test_tags_matches_no_slash() {
        let uri = "/v2/nginx/tags/list";
        let captures = URI_NAME_REGEX.captures(uri);
        assert_eq!(captures.is_some(), true);
        let captures = captures.unwrap();
        assert_eq!(captures.name("name").unwrap().as_str(), "nginx");
        assert_eq!(captures.name("resource").unwrap().as_str(), "tags");
    }

    #[test]
    fn test_tags_matches_with_slash() {
        let uri = "/v2/library/nginx/tags/list";
        let captures = URI_NAME_REGEX.captures(uri);
        assert_eq!(captures.is_some(), true);
        let captures = captures.unwrap();
        assert_eq!(captures.name("name").unwrap().as_str(), "library/nginx");
        assert_eq!(captures.name("resource").unwrap().as_str(), "tags");
    }
}
