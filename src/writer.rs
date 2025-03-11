/*Write and delete files present on the server if it exist and if the user is authorized */
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use tokio::{fs, io};

use crate::auth::User;
use crate::utils::{root_path, trim_path , path_of, get_extention};

pub async fn rename_or_fallback(path: &str, name : &str, user: &Option<User>) -> Response {
    //if user connected
    if let Some(_u) = &*user {
        if root_path(path) {
            (
                StatusCode::UNAUTHORIZED,
                format!("Cannot rename that dir\n"),
            )
                .into_response()
        } else if let Ok(r) = rename(path , name).await {
            r
        } else {
            (StatusCode::NOT_FOUND, format!("Cannot rename {path}\n")).into_response()
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

pub async fn remove_or_fallback(path: &str, user: &Option<User>) -> Response {
    //if user connected
    if let Some(_u) = &*user {
        if root_path(path) {
            (
                StatusCode::UNAUTHORIZED,
                format!("Cannot remove that dir\n"),
            )
                .into_response()
        } else if let Ok(r) = remove(path).await {
            r
        } else {
            (StatusCode::NOT_FOUND, format!("Cannot find {path}\n")).into_response()
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
async fn rename(path: &str , name : &str) -> io::Result<Response> {
        if let Some(p) = path_of(path) {
            let to_write = match get_extention(path) {
                Some(ext) => format!("{}/{}.{}" , p , trim_path(name) , ext),
                //is directory or no_file extention
                None => format!("{}/{}" , p , trim_path(name))
            };
            let trimmed = trim_path(path);
            fs::rename(&trimmed , &to_write ).await?;
            Ok((
                StatusCode::OK,
                format!("ressource : {} renamed successfully to {}\n", path , &to_write),
            )
                .into_response())
        }
        //cannot find '/' => invalid path
        else {
            Ok((
                StatusCode::BAD_REQUEST,
                format!("Invalid path : {path}\n"),
            )
                .into_response())
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

pub async fn write_file_or_fallback(path: &str, content: &[u8]) -> Response {
    if root_path(path) {
        (
            StatusCode::UNAUTHORIZED,
            format!("Cannot write file here\n"),
        )
            .into_response()
    } else if let Ok(r) = write_file(path, content).await {
        r
    } else {
        (StatusCode::BAD_REQUEST, format!("Cannot write {path}\n")).into_response()
    }
}

async fn write_file(path: &str, content: &[u8]) -> io::Result<Response> {
    fs::write(path, content).await?;
    Ok((
        StatusCode::OK,
        format!("file : {} written successfully\n", path),
    )
        .into_response())
}

pub async fn write_dir_or_fallback(path: &str) -> Response {
    let trimmed = trim_path(path);
    if root_path(path) {
        (StatusCode::UNAUTHORIZED, format!("Cannot write dir here\n")).into_response()
    } else if let Ok(r) = write_dir(&trimmed).await {
        r
    } else {
        (StatusCode::BAD_REQUEST, format!("Cannot write {path}\n")).into_response()
    }
}

async fn write_dir(path: &str) -> io::Result<Response> {
    fs::create_dir(path).await?;
    Ok((
        StatusCode::OK,
        format!("Dir : {} written successfully\n", path),
    )
        .into_response())
}
