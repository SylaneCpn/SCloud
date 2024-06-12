/*Provide ressource for the web app, and content that does not require auth*/

//###########################################################################################//

use axum::{extract::Path, response::Response};

use crate::reader::respond_or_fallback;

//###########################################################################################//


//provide assets for the web app such as images , videos , css ,etc ... Does not require any auth
pub async fn assets(Path(path): Path<String>) -> Response {
    let complete_path = format!("assets/{}", path);
    respond_or_fallback(&complete_path).await
}
