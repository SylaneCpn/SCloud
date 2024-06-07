use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::response_provider::respond;

pub async fn raw(Path(path): Path<String>) -> Response {
    let complete_path = format!("files/{}", path);
    if let Ok(r) = respond(&complete_path).await {
        r
    } else {
        //fallback
        (StatusCode::NOT_FOUND, "Cannot find path...").into_response()
    }
}
