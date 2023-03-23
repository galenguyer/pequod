//! List a set of available repositories in the local registry cluster.
//! Does not provide any indication of what may be available upstream.
//! Applications can only determine if a repository is available but not if it is not available.

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

/// Retrieve a sorted, json list of repositories available in the registry.
/// ##  Catalog Fetch
/// ```txt
/// GET /v2/_catalog
/// ```
///
/// Request an unabridged list of repositories available.  The implementation may impose a maximum limit and return a partial set with pagination links.
///
/// ### On Success: OK
///
/// ```txt
/// 200 OK
/// Content-Length: <length>
/// Content-Type: application/json
///
/// {
///     "repositories": [
///         <name>,
///         ...
///     ]
/// }
/// ```
///
/// Returns the unabridged list of repositories as a json response.
///
/// The following headers will be returned with the response:
///
/// |Name|Description|
/// |----|-----------|
/// |`Content-Length`|Length of the JSON response body.|
///
///
/// ## Catalog Fetch Paginated
///
/// ```txt
/// GET /v2/_catalog?n=<integer>&last=<last repository value from previous response>
/// ```
///
/// Return the specified portion of repositories.
///
///
/// The following parameters should be specified on the request:
///
/// |Name|Kind|Description|
/// |----|----|-----------|
/// |`n`|query|Limit the number of entries in each response. It not present, 100 entries will be returned.|
/// |`last`|query|Result set will include values lexically after last.|
///
///
/// ### On Success: OK
///
/// ```txt
/// 200 OK
/// Content-Length: <length>
/// Link: <<url>?n=<last n value>&last=<last entry from response>>; rel="next"
/// Content-Type: application/json
///
/// {
///     "repositories": [
///         <name>,
///         ...
///     ]
///     "next": "<url>?last=<name>&n=<last value of n>"
/// }
/// ```
///
///
///
/// The following headers will be returned with the response:
///
/// |Name|Description|
/// |----|-----------|
/// |`Content-Length`|Length of the JSON response body.|
/// |`Link`|RFC5988 compliant rel='next' with URL to next result set, if available|
///
/// [Reference](https://docs.docker.com/registry/spec/api/#get-catalog)
pub async fn catalog() -> impl IntoResponse {
    let repos = crate::db::sqlite::repositories::list().await;
    let names = repos
        .unwrap()
        .iter()
        .map(|r| r.name.clone())
        .collect::<Vec<String>>();
    (StatusCode::OK, Json(json!({ "repositories": names })))
}
