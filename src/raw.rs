use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::response_provider::respond;

//provide the server's files and directories  for the web. require auth
pub async fn raw(Path(path): Path<String>) -> Response {
    let complete_path = format!("files/{}", path);
    if let Ok(r) = respond(&complete_path).await {
        r
    } else {
        //fallback
        (StatusCode::NOT_FOUND, "Cannot find path...").into_response()
    }
}
