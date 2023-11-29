use anyhow::anyhow;
use axum::body::Bytes;
use axum::response::Response;
use http_body_util::BodyExt;

/// Extract the response body bytes.
pub async fn extract_body(response: Response) -> anyhow::Result<Bytes> {
    let (_, body) = response.into_parts();
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(why) => return Err(anyhow!(why.to_string())),
    };

    Ok(bytes)
}
