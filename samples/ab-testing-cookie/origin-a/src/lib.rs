use bytes::Bytes;
use spin_sdk::http::{FullBody, IntoResponse, Method, Request, Response, StatusCode};
use spin_sdk::http_service;

#[http_service]
async fn handle_origin_a(req: Request) -> anyhow::Result<impl IntoResponse> {
    if req.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(FullBody::default())
            .unwrap());
    }
    Ok(match req.uri().path().to_lowercase().as_str() {
        "/origin-a/by-cookie.html" => send_page(),
        _ => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(FullBody::default())
            .unwrap(),
    })
}

fn send_page() -> Response<FullBody<Bytes>> {
    let page = include_str!("by-cookie.html");
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/html")
        .body(FullBody::new(Bytes::from(page)))
        .unwrap()
}
