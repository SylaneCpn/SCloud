use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::auth::{check_user, verify_access};
use crate::response_provider::respond_or_fallback;

//provide the server's files and directories  for the web. require auth
pub async fn files(Path((user, password, path)): Path<(String, String, String)>) -> Response {
    let complete_path = format!("files/{}", &path);

    //check if user is in the database
    if let Some(u) = check_user(&user, &password) {
        //verify if the user has access to the source
        if verify_access(&u, &path) {
            //respond if access granted
            respond_or_fallback(&complete_path).await
        } else {
            //respond to unauthorised user
            (
                StatusCode::UNAUTHORIZED,
                format!("Not Authorized"),
            )
                .into_response()
        }
    }
    //if user doesn't exist give him access anyway if it's from the public repo
    else if path.starts_with("public") {
        respond_or_fallback(&complete_path).await
    }
    //respond to unauthorised user
    else {
        (
            StatusCode::UNAUTHORIZED,
            format!("Not Authorized"),
        )
            .into_response()
    }
}
