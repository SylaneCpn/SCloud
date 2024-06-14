/*Write and delete files present on the server if it exist and if the user is authorized */
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use tokio::{fs, io};

use crate::auth::User;
use crate::utils::trim_path;

pub async fn remove_or_fallback(path: &str, user: &Option<User>) -> Response {
    //if user connected
    if let Some(_u) = &*user {
        let trimmed = trim_path(path);
        //check if those are users directorires
        let path_dept = trimmed.split("/").map(|_| 1).sum::<usize>();
        //cannot remove root diectories
        if path_dept == 2 {
            (
                StatusCode::UNAUTHORIZED,
                format!("Cannot remove that dir\n"),
            )
                .into_response()
        } else {
            if let Ok(r) = remove(path).await {
                r
            } else {
                (StatusCode::NOT_FOUND, format!("Cannot find {path}\n")).into_response()
            }
        }
    }
    //guests cannot remove files
    else {
        (
            StatusCode::UNAUTHORIZED,
            format!("Cannot remove files as a guest\n"),
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
            format!("dir : {} removed successfully\n", path),
        )
            .into_response())
    }
    //assume file
    else {
        fs::remove_file(path).await?;
        Ok((
            StatusCode::OK,
            format!("file : {} removed successfully\n", path),
        )
            .into_response())
    }
}

pub async fn write_file_or_fallback(path : &str , content : &[u8]) -> Response {

    if let Ok(r) = write_file(path , content).await {
        r
    }

    else {
        (StatusCode::BAD_REQUEST, format!("Cannot write {path}\n")).into_response()
    }

}

async fn write_file(path : &str , content : &[u8]) -> io::Result<Response> {
    fs::write(path,content).await?;
    Ok((
        StatusCode::OK,
        format!("file : {} written successfully\n", path),
    )
        .into_response())
}


pub async fn write_dir_or_fallback(path : &str) -> Response {
    let trimmed = trim_path(path);
    if let Ok(r) = write_dir(&trimmed).await {
        r
    }

    else {
        (StatusCode::BAD_REQUEST, format!("Cannot write {path}\n")).into_response()
    }

}


async fn write_dir(path : &str) -> io::Result<Response> {
    fs::create_dir(path).await?;
    Ok((
        StatusCode::OK,
        format!("Dir : {} written successfully\n", path),
    )
        .into_response())
}