/*Handle post and delete requests that manipulate the files of the server, require auth */

//###########################################################################################//
use axum::{
    body::Bytes,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::auth::{check_user, verify_access};
use crate::writer::{remove_or_fallback, write_dir_or_fallback, write_file_or_fallback};

pub async fn create_file(
    Path((user, password, path)): Path<(String, String, String)>,
    body: Bytes,
) -> Response {
    let complete_path = format!("files/{}", &path);
    let content = body.to_vec();
    let u = check_user(&user, &password).await;
    if verify_access(&u, &path) {
        //respond if access granted
        write_file_or_fallback(&complete_path, &content).await
    } else {
        //respond to unauthorised user
        (StatusCode::UNAUTHORIZED, format!("Not Authorized\n")).into_response()
    }
}

pub async fn create_dir(Path((user, password, path)): Path<(String, String, String)>) -> Response {
    let complete_path = format!("files/{}", &path);
    let u = check_user(&user, &password).await;
    if verify_access(&u, &path) {
        //respond if access granted
        write_dir_or_fallback(&complete_path).await
    } else {
        //respond to unauthorised user
        (StatusCode::UNAUTHORIZED, format!("Not Authorized\n")).into_response()
    }
}

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
