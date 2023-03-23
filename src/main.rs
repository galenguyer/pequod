use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::{routing, Router, ServiceExt};
use lazy_static::lazy_static;
use regex::Regex;
use tower::Layer;

pub mod api;
pub mod db;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"/v2/(?P<name>[\w/]+)/(?P<resource>(tags|manifests|blobs))/").unwrap();
}

async fn rewrite_request_uri<B>(mut req: Request<B>, next: Next<B>) -> Response {
    let captures = match RE.captures(req.uri().path()) {
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
async fn main() {
    let rewriter = axum::middleware::from_fn(rewrite_request_uri);
    let router = Router::new().nest(
        "/v2",
        Router::new()
            .route("/", routing::get(api::base))
            .route("/_catalog", routing::get(api::catalog))
            .route("/:name/tags/list", routing::get(api::tags)),
    );

    let app = rewriter.layer(router);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod test {
    use super::RE;

    #[test]
    fn test_tags_matches_no_slash() {
        let uri = "/v2/nginx/tags/list";
        let captures = RE.captures(uri);
        assert_eq!(captures.is_some(), true);
        let captures = captures.unwrap();
        assert_eq!(captures.name("name").unwrap().as_str(), "nginx");
        assert_eq!(captures.name("resource").unwrap().as_str(), "tags");
    }

    #[test]
    fn test_tags_matches_with_slash() {
        let uri = "/v2/library/nginx/tags/list";
        let captures = RE.captures(uri);
        assert_eq!(captures.is_some(), true);
        let captures = captures.unwrap();
        assert_eq!(captures.name("name").unwrap().as_str(), "library/nginx");
        assert_eq!(captures.name("resource").unwrap().as_str(), "tags");
    }
}
