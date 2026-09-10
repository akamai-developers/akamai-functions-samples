use anyhow::*;
use bytes::Bytes;
use graphql_client::GraphQLQuery;
use spin_sdk::http::body::IncomingBodyExt;
use spin_sdk::http::{FullBody, IntoResponse, Method, Request, Response, StatusCode, send};
use spin_sdk::http_service;

#[allow(clippy::upper_case_acronyms)]
type URI = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/query_1.graphql",
    response_derives = "Debug"
)]
struct RepoView;

/// A simple Spin HTTP component.
#[http_service]
async fn handle_graphql(req: Request) -> Result<impl IntoResponse> {
    let path = req.uri().path();

    // Serve the landing page with the owner/repo lookup form.
    if path == "/" || path == "/index.html" {
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "text/html")
            .body(FullBody::new(Bytes::from(include_str!("index.html"))))
            .unwrap());
    }

    // Anything that isn't a "/stargazers/owner/repo" path (including browser
    // requests like GET /favicon.ico) is sent back to the landing page.
    let Some((owner, name)) = parse_repo_name(path).ok() else {
        return Ok(Response::builder()
            .status(StatusCode::FOUND)
            .header("location", "/")
            .body(FullBody::default())
            .unwrap());
    };

    let github_api_token = spin_sdk::variables::get("gh_api_token")
        .await
        .expect("Missing gh_api_token variable");

    let variables = repo_view::Variables {
        owner: owner.to_string(),
        name: name.to_string(),
    };

    let body = RepoView::build_query(variables);
    let body = serde_json::to_string(&body).unwrap();

    let outgoing = Request::builder()
        .method(Method::POST)
        .uri("https://api.github.com/graphql")
        .header("user-agent", "graphql-rust")
        .header("content-type", "application/json")
        .header("Authorization", format!("Bearer {}", github_api_token))
        .body(FullBody::new(Bytes::from(body)))
        .unwrap();
    let res: Response = send(outgoing).await?;

    let res_bytes = res.into_body().bytes().await?;
    let response: graphql_client::Response<repo_view::ResponseData> =
        serde_json::from_slice(&res_bytes).unwrap();
    let response_data = response.data.expect("missing response data");

    let stars = response_data
        .repository
        .as_ref()
        .map(|repo| repo.stargazer_count);
    match stars {
        Some(stars) => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "text/html")
            .body(FullBody::new(Bytes::from(render_stargazers(
                stars, owner, name,
            ))))
            .unwrap()),
        None => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("content-type", "text/plain")
            .body(FullBody::new(Bytes::from(format!(
                "Repository {}/{} not found",
                owner, name
            ))))
            .unwrap()),
    }
}

fn parse_repo_name(path: &str) -> Result<(&str, &str), anyhow::Error> {
    let mut parts = path.split('/').filter(|s| !s.is_empty());
    match (parts.next(), parts.next(), parts.next()) {
        (Some("stargazers"), Some(owner), Some(name)) => Ok((owner, name)),
        _ => Err(format_err!(
            "wrong format for the repository path (we expect something like /stargazers/spinframework/spin)"
        )),
    }
}

fn render_stargazers(stars: i64, org: &str, repo: &str) -> String {
    include_str!("stargazers.html")
        .replace("__STARS__", &stars.to_string())
        .replace("__ORG__", org)
        .replace("__REPO__", repo)
}

