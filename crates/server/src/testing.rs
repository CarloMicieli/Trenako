use anyhow::anyhow;
use axum::response::Response;
use hyper::body::Bytes;

/// Extract the response body bytes.
pub async fn extract_body(response: Response) -> anyhow::Result<Bytes> {
    let (_, body) = response.into_parts();
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(why) => return Err(anyhow!(why.to_string())),
    };

    Ok(bytes)
}
