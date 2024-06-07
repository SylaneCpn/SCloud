use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::response_provider::respond;

pub async fn assets(Path(path): Path<String>) -> Response {
    let complete_path = format!("assets/{}", path);
    if let Ok(r) = respond(&complete_path).await {
        r
    } else {
        //fallback
        (
            StatusCode::NOT_FOUND,
            format!("Cannot find {complete_path}"),
        )
            .into_response()
    }
}
