use bytes::Bytes;
use spin_sdk::http::body::IncomingBodyExt;
use spin_sdk::http::{
    EmptyBody, FullBody, IntoResponse, Method, Request, Response, StatusCode, send,
};
use spin_sdk::http_service;

const ORIGIN_REQUEST_PATH: &str = "by-user-agent.html";
const ORIGIN_A: &str = "origin-a";
const ORIGIN_B: &str = "origin-b";

#[http_service]
async fn handle_ab_testing(req: Request) -> anyhow::Result<impl IntoResponse> {
    if req.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(FullBody::default())
            .unwrap());
    }
    let res = match req.uri().path().to_lowercase().as_str() {
        "/index.html" => send_index(),
        "/by-user-agent" => send_a_or_b(&req).await,
        "/" => send_redirect_to("/index.html"),
        _ => send_not_found(),
    };
    Ok(res)
}

fn send_redirect_to(target: &str) -> Response<FullBody<Bytes>> {
    Response::builder()
        .status(301)
        .header("location", target)
        .body(FullBody::default())
        .unwrap()
}

fn send_not_found() -> Response<FullBody<Bytes>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(FullBody::default())
        .unwrap()
}

fn send_index() -> Response<FullBody<Bytes>> {
    let index = include_str!("index.html");
    Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(FullBody::new(Bytes::from(index)))
        .unwrap()
}

async fn send_a_or_b(req: &Request) -> Response<FullBody<Bytes>> {
    let user_agent = match req.headers().get("user-agent") {
        Some(header) => match header.to_str() {
            Ok(value) => value.to_string(),
            Err(_) => return send_bad_request("user-agent header is empty"),
        },
        None => return send_bad_request("user-agent header not present"),
    };

    println!("{user_agent}");

    let origin = if user_agent.contains("Chrome") || user_agent.contains("Safari") {
        ORIGIN_B
    } else {
        ORIGIN_A
    };

    let origin_req = Request::builder()
        .method(Method::GET)
        .uri(build_request_url(req, origin))
        .body(EmptyBody::new())
        .unwrap();
    let origin_response: Response = send(origin_req).await.unwrap();

    let response_bytes = origin_response.into_body().bytes().await.unwrap();
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/html")
        .body(FullBody::new(response_bytes))
        .unwrap()
}

fn send_bad_request(message: &str) -> Response<FullBody<Bytes>> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(FullBody::new(Bytes::from(message.to_string())))
        .unwrap()
}

fn build_request_url(incoming_req: &Request, outgoing_simulated_origin: &str) -> String {
    format!(
        "{}://self.alt/{outgoing_simulated_origin}/{ORIGIN_REQUEST_PATH}",
        incoming_req.uri().scheme_str().unwrap_or("https"),
    )
}
