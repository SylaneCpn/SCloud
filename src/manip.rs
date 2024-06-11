use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use tokio::{fs, io};

use crate::auth::{check_user, verify_access, User};
use crate::response_provider::{slash_path, trim_path};

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
        (StatusCode::UNAUTHORIZED, format!("Not Authorized")).into_response()
    }
}

async fn remove_or_fallback(path: &str, user: &Option<User>) -> Response {
    //if user connected
    if let Some(_u) = &*user {
        let trimmed = trim_path(path);
        //check if those are users directorires
        let path_dept = trimmed.split("/").map(|_| 1).sum::<usize>();
        //cannot remove root diectories
        if path_dept == 2 {
            (StatusCode::UNAUTHORIZED, format!("Cannot remove that dir")).into_response()
        } else {
            if let Ok(r) = remove(path).await {
                r
            } else {
                (StatusCode::NOT_FOUND, format!("Cannot find {path}")).into_response()
            }
        }
    }
    //guests cannot remove files
    else {
        (
            StatusCode::UNAUTHORIZED,
            format!("Cannot remove files as a guest"),
        )
            .into_response()
    }
}

//remove the ressorce at the path
async fn remove(path: &str) -> io::Result<Response> {
    let md = fs::metadata(path).await?;
    //is a dir ?
    if md.is_dir() {
        fs::remove_dir_all(path).await?;
        Ok((
            StatusCode::OK,
            format!("dir : {} removed successfully", path),
        )
            .into_response())
    }
    //assume file
    else {
        fs::remove_file(path).await?;
        Ok((
            StatusCode::OK,
            format!("file : {} removed successfully", path),
        )
            .into_response())
    }
}
