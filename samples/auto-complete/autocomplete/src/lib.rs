use bytes::Bytes;
use serde::{Deserialize, Serialize};
use spin_sdk::http::body::IncomingBodyExt;
use spin_sdk::http::{EmptyBody, FullBody, IntoResponse, Method, Request, Response, send};
use spin_sdk::http_service;
use std::collections::HashMap;
use url::form_urlencoded;

#[http_service]
async fn handle_autocomplete(req: Request) -> anyhow::Result<impl IntoResponse> {
    let query = req.uri().query().unwrap_or_default();

    // API endpoint: /?term=<value> returns JSON suggestions from the dataset
    // baked into the Wasm module, or falls back to the origin when unknown.
    // Parse the query properly so percent-encoded terms (e.g. "BMW 3" arriving
    // as "BMW%203") are decoded before the lookup.
    let search_term = form_urlencoded::parse(query.as_bytes())
        .find(|(key, _)| key == "term")
        .map(|(_, value)| value.into_owned());
    if let Some(search_term) = search_term {
        let data = get_autocomplete_sample_data();
        return match data.get(search_term.to_lowercase().as_str()) {
            Some(value) => {
                let payload = serde_json::to_string(value).unwrap();
                Ok(Response::builder()
                    .status(200)
                    .header("content-type", "application/json")
                    .body(FullBody::new(Bytes::from(payload)))
                    .unwrap())
            }
            None => {
                let scheme = req.uri().scheme_str().unwrap_or("https");
                let path_and_query = req
                    .uri()
                    .path_and_query()
                    .map(|pq| pq.as_str())
                    .unwrap_or_default();
                let origin_url = format!("{scheme}://self.alt/origin/{path_and_query}");
                let origin_request = Request::builder()
                    .method(Method::GET)
                    .uri(origin_url)
                    .body(EmptyBody::new())
                    .unwrap();
                let origin_response = send(origin_request).await?;

                // Forward the origin response verbatim (status, headers, and body).
                let status = origin_response.status();
                let headers = origin_response.headers().clone();
                let body = origin_response.into_body().bytes().await?;
                let mut builder = Response::builder().status(status);
                for (name, value) in headers.iter() {
                    builder = builder.header(name, value);
                }
                Ok(builder.body(FullBody::new(body)).unwrap())
            }
        };
    }

    // Otherwise serve the demo UI that drives the endpoint above.
    let path = req.uri().path();
    if path == "/" || path == "/index.html" {
        return Ok(Response::builder()
            .status(200)
            .header("content-type", "text/html")
            .body(FullBody::new(Bytes::from(include_str!("index.html"))))
            .unwrap());
    }

    Ok(Response::builder()
        .status(404)
        .header("content-type", "text/plain")
        .body(FullBody::new(Bytes::from("Not found")))
        .unwrap())
}

fn get_autocomplete_sample_data() -> HashMap<String, Vec<AutoCompleteData>> {
    let bytes = include_bytes!("../data/sample-set.json");
    serde_json::from_slice(bytes).unwrap()
}

#[derive(Deserialize, Serialize)]
pub(crate) struct AutoCompleteData {
    pub label: String,
    pub value: String,
}
