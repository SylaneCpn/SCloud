use serde::{Deserialize, Serialize};
use tokio::fs;

use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub password: String,
    pub admin: bool,
}
pub async fn verify_user(Path((user, password)): Path<(String, String)>) -> Response {
    if let Some(u) = check_user(&user, &password).await {
        format!("User {} verified", u.name).into_response()
    } else {
        (StatusCode::NOT_FOUND, format!("User does not exist/Bad password")).into_response()
    }
}

pub async fn check_user(name: &str, password: &str) -> Option<User> {
    let mut usr = None;
    let content = fs::read_to_string("users.json").await.unwrap();
    let users = serde_json::from_str::<Vec<User>>(&content).unwrap();

    for user in users.iter() {
        if user.name == name && user.password == password {
            usr = Some(user.clone());
            break;
        }
    }
    usr
}

pub fn verify_access(user: &User, path: &str) -> bool {
    if user.admin {
        //grant access if admin
        true
    } else if path.starts_with("public/") || path.starts_with(&format!("{}/", user.name)) {
        //user has access to the repo
        true
    } else {
        //unauthorised
        false
    }
}
