use bytes::Bytes;
use spin_sdk::http::body::IncomingBodyExt;
use spin_sdk::http::{
    EmptyBody, FullBody, IntoResponse, Method, Request, Response, StatusCode, send,
};
use spin_sdk::http_service;
use spin_sdk::key_value::Store;

const ORIGIN_REQUEST_PATH: &str = "by-key-value.html";
const ORIGIN_A: &str = "origin-a";
const ORIGIN_B: &str = "origin-b";
const COUNTER_KEY: &str = "counter";

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
        "/by-key-value" => send_a_or_b(&req).await,
        _ => send_redirect_to("/index.html"),
    };
    Ok(res)
}

fn send_redirect_to(target: &str) -> Response<FullBody<Bytes>> {
    Response::builder()
        .status(308)
        .header("location", target)
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
    // Increment a counter in the default key-value store on every request.
    // Odd counts are routed to origin-a, even counts to origin-b, so
    // consecutive requests alternate between Variant A and Variant B.
    let store = Store::open_default().await.unwrap();
    let mut count = match store.get(COUNTER_KEY).await.unwrap() {
        Some(v) => u32::from_le_bytes(v.try_into().unwrap_or_default()),
        None => 0,
    };
    count += 1;
    store
        .set(COUNTER_KEY, count.to_le_bytes().as_slice())
        .await
        .unwrap();

    let origin_url = match count % 2 == 0 {
        true => build_request_url(req, ORIGIN_B),
        false => build_request_url(req, ORIGIN_A),
    };
    let origin_req = Request::builder()
        .method(Method::GET)
        .uri(origin_url)
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

fn build_request_url(incoming_req: &Request, outgoing_simulated_origin: &str) -> String {
    format!(
        "{}://self.alt/{outgoing_simulated_origin}/{ORIGIN_REQUEST_PATH}",
        incoming_req.uri().scheme_str().unwrap_or("https"),
    )
}
