use anyhow::Context;
use bytes::Bytes;
use spin_sdk::http::body::IncomingBodyExt;
use spin_sdk::http::{
    BoxBody, EmptyBody, FullBody, HeaderName, HeaderValue, IntoResponse, Method, Request, Response,
    StatusCode, box_body, send,
};
use spin_sdk::http_service;

/// A simple Spin HTTP component.
#[http_service]
async fn handle_rewrite_for_bots_price_hiding(req: Request) -> anyhow::Result<impl IntoResponse> {
    let upstream_host = spin_sdk::variables::get("upstream_host")
        .await
        .expect("upstream_host must be set");
    let path_and_query = req
        .uri()
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or("/");
    let upstream_url = format!("{upstream_host}{path_and_query}");

    // FOR DEMONSTRATION PURPOSES: check for ?content-type=bot query param
    // In a real deployment, requests would only come to the rewriter if the CDN
    // already determined them to be bots.
    let is_bot_request = req
        .uri()
        .query()
        .map(|q| {
            url::form_urlencoded::parse(q.as_bytes()).any(|(k, v)| k == "content-type" && v == "bot")
        })
        .unwrap_or(false);

    // If this is NOT a bot request, pass it through. Again, in a production deployment
    // this would not be necessary because the function would never see non-bot requests.
    if !is_bot_request {
        let method = req.method().clone();
        let headers: Vec<(HeaderName, HeaderValue)> = req
            .headers()
            .iter()
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect();
        let request_body = req.into_body().bytes().await?;

        let mut builder = Request::builder().method(method).uri(&upstream_url);
        for (name, value) in headers {
            builder = builder.header(name, value);
        }
        let passthrough_request = builder.body(FullBody::new(request_body))?;
        let passthrough_response = send(passthrough_request).await?;
        return Ok(box_response(passthrough_response));
    }

    // Bot protection processing begins here

    let error_response = spin_sdk::variables::get("error_response")
        .await
        .expect("error_response should have been set");
    let replacements_json = spin_sdk::variables::get("replacements_json")
        .await
        .expect("replacements_json should have been set");
    let replacements: Vec<Replacement> =
        serde_json::from_str(&replacements_json).context("invalid replacements JSON")?;

    let upstream_request = Request::builder()
        .method(Method::GET)
        .uri(&upstream_url)
        .header("user-agent", "FWF Price Hiding")
        .body(EmptyBody::new())?;

    let upstream_response = match send(upstream_request).await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Bot response not generated: upstream request for {upstream_url} failed: {e:?}");
            return Ok(error_response_with(403, &error_response));
        }
    };

    let status = upstream_response.status();

    // Not found, gone, or redirected
    if status == StatusCode::NOT_FOUND
        || status == StatusCode::GONE
        || status == StatusCode::MOVED_PERMANENTLY
        || status == StatusCode::TEMPORARY_REDIRECT
    {
        return Ok(box_response(upstream_response));
    }

    if !status.is_success() {
        eprintln!("Bot response not generated: upstream request for {upstream_url} returned status code {status}");
        return Ok(error_response_with(503, &error_response));
    }

    let content_type = upstream_response
        .headers()
        .get("content-type")
        .and_then(|hval| hval.to_str().ok())
        .map(|s| s.to_owned());

    // Only text responses can be rewritten. Non-text responses are passed through untouched.
    let is_text = match content_type.as_deref() {
        Some(ct) => ct.starts_with("text/") || ct == "application/html",
        None => true,
    };
    if !is_text {
        eprintln!("Bot response not generated: upstream request for {upstream_url} returned non-text response");
        return Ok(box_response(upstream_response));
    }

    let body_bytes = upstream_response.into_body().bytes().await?;

    let Ok(response_text) = String::from_utf8(body_bytes.to_vec()) else {
        // Body was not valid UTF-8: pass the original content through untouched.
        return Ok(passthrough_bytes(status, content_type.as_deref(), body_bytes));
    };

    let protected_text = match hide_prices(&response_text, &replacements) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Bot response not generated: processing response from {upstream_url} failed: {e:?}");
            return Ok(error_response_with(503, &error_response));
        }
    };

    Ok(passthrough_bytes(
        StatusCode::OK,
        Some(content_type.as_deref().unwrap_or("text/html")),
        Bytes::from(protected_text),
    ))
}

/// Wraps an incoming (streaming) response so its body type matches the other
/// return paths of the handler.
fn box_response(response: Response) -> Response<BoxBody> {
    let (parts, body) = response.into_parts();
    Response::from_parts(parts, box_body(body))
}

/// Builds a fully-buffered response with the given status, optional content-type
/// header, and body.
fn passthrough_bytes(
    status: StatusCode,
    content_type: Option<&str>,
    body: Bytes,
) -> Response<BoxBody> {
    let mut builder = Response::builder().status(status);
    if let Some(ct) = content_type {
        builder = builder.header("content-type", ct);
    }
    builder.body(box_body(FullBody::new(body))).unwrap()
}

/// Builds a plain-text error response.
fn error_response_with(status: u16, message: &str) -> Response<BoxBody> {
    Response::builder()
        .status(status)
        .header("content-type", "text/plain")
        .body(box_body(FullBody::new(Bytes::from(message.to_owned()))))
        .unwrap()
}

fn hide_prices(html: &str, replacements: &[Replacement]) -> anyhow::Result<String> {
    fn replace_with(
        replacment: String,
    ) -> impl Fn(
        &mut lol_html::html_content::Element,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        move |el: &mut lol_html::html_content::Element| -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            el.set_inner_content(&replacment, lol_html::html_content::ContentType::Html);
            Ok(())
        }
    }

    let element_content_handlers = replacements
        .iter()
        .flat_map(|r| {
            r.selectors
                .iter()
                .map(|s| lol_html::element!(s, replace_with(r.replacement.clone())))
        })
        .collect();

    let settings = lol_html::Settings {
        element_content_handlers,
        ..lol_html::Settings::new()
    };

    let r = lol_html::rewrite_str(html, settings)?;
    Ok(r)
}

#[derive(serde::Deserialize)]
struct Replacement {
    selectors: Vec<String>,
    replacement: String,
}
