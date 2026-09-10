use bytes::Bytes;
use spin_sdk::http::{FullBody, IntoResponse, Request, Response};
use spin_sdk::http_service;
use spin_sdk::key_value::Store;

#[http_service]
async fn handle_origin(_req: Request) -> anyhow::Result<impl IntoResponse> {
    let store = Store::open_default().await?;
    let mut count = match store.get("count").await? {
        Some(v) => u32::from_le_bytes(v.try_into().unwrap()),
        None => 0,
    };

    count += 1;
    store.set("count", count.to_le_bytes().as_slice()).await?;
    Ok(Response::builder()
        .status(200)
        .header("X-Origin-Request-Count", format!("{count}"))
        .body(FullBody::new(Bytes::from(
            "This content has been returned from the origin",
        )))
        .unwrap())
}
