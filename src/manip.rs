/*Handle post and delete requests that manipulate the files of the server, require auth */

//###########################################################################################//
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::auth::{check_user, verify_access};
use crate::writer::remove_or_fallback;

pub async fn create_file(Path((user, password, path)): Path<(String, String, String)>) {}

pub async fn remove_ressource(
    Path((user, password, path)): Path<(String, String, String)>,
) -> Response {
    let complete_path = format!("files/{}", &path);

    //check if user is in the database
    let u = check_user(&user, &password).await;
    //verify if the user has access to the source
    if verify_access(&u, &path) {
        //respond if access granted
        remove_or_fallback(&complete_path, &u).await
    } else {
        //respond to unauthorised user
        (StatusCode::UNAUTHORIZED, format!("Not Authorized\n")).into_response()
    }
}
