use bytes::Bytes;
use lol_html::{RewriteStrSettings, element, rewrite_str};
use spin_sdk::http::body::IncomingBodyExt;
use spin_sdk::http::{
    EmptyBody, FullBody, IntoResponse, Method, Request, Response, StatusCode, send,
};
use spin_sdk::http_service;
use spin_sdk::key_value::Store;

use crate::cache::CacheItem;
use crate::config::Config;

mod cache;
mod config;

#[http_service]
async fn handle_html_rewrite(req: Request) -> anyhow::Result<impl IntoResponse> {
    let Ok(cfg) = Config::load().await else {
        println!("Application Configuration missing or invalid");
        return Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(FullBody::default())
            .unwrap());
    };

    let url = build_upstream_url(&req, cfg.upstream_url.clone());

    if cfg.use_key_value_store {
        let kv = Store::open_default().await?;
        let key = url.as_str();
        let found = CacheItem::load_by_key(&kv, key).await?;
        if let Some(found) = found {
            if found.is_valid(cfg.ttl_in_minutes) {
                println!("Cache hit: Item is still valid");
                return Ok(html_response(rewrite_html(found.value)));
            } else {
                println!("Cache hit: Item is expired");
                CacheItem::delete_by_key(&kv, key).await?;
            }
        }
    }

    let origin_req = Request::builder()
        .method(Method::GET)
        .uri(url.clone())
        .body(EmptyBody::new())
        .unwrap();
    let response: Response = send(origin_req).await?;

    // Capture status and headers before consuming the body so we can forward
    // the upstream response as-is when it should not be rewritten.
    let status = response.status();
    let is_html = response
        .headers()
        .get("content-type")
        .and_then(|hv| hv.to_str().ok())
        .is_some_and(|v| v.contains("text/html"));
    let headers = response.headers().clone();

    let body_bytes = response.into_body().bytes().await?;

    // if we receive a non 200 status code
    // or if the content type is not text/html
    // return the response as is
    if status != StatusCode::OK || !is_html {
        println!("Either status != 200 or non-html");
        let mut builder = Response::builder().status(status);
        if let Some(dst) = builder.headers_mut() {
            *dst = headers;
        }
        return Ok(builder.body(FullBody::new(body_bytes)).unwrap());
    }

    let input = String::from_utf8_lossy(&body_bytes).to_string();

    if cfg.use_key_value_store {
        let kv = Store::open_default().await?;
        let item = CacheItem::new(input.clone());
        item.store_at_key(&kv, &url).await?;
    }
    Ok(html_response(rewrite_html(input)))
}

fn html_response(html: String) -> Response<FullBody<Bytes>> {
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/html")
        .body(FullBody::new(Bytes::from(html)))
        .unwrap()
}

fn rewrite_html(input: String) -> String {
    let element_content_handlers = vec![element!("h1", |el| {
        el.set_inner_content(
            "Hello Bot Protection this is Akamai Functions",
            lol_html::html_content::ContentType::Text,
        );
        Ok(())
    })];

    rewrite_str(
        &input,
        RewriteStrSettings {
            element_content_handlers,
            ..RewriteStrSettings::new()
        },
    )
    .unwrap_or(input)
}

fn build_upstream_url(req: &Request, upstream_base_url: String) -> String {
    let path = req
        .uri()
        .path_and_query()
        .map(|pq| pq.as_str())
        .filter(|&s| !s.is_empty() && s != "/")
        .unwrap_or("/index.html");
    format!("{}{}", upstream_base_url, path)
}
