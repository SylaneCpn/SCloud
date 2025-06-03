/*Handle auth for the app*/

//###########################################################################################//

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
//###########################################################################################//
//let the app know if the user is correct or not

pub async fn verify_user(Path((user, password)): Path<(String, String)>) -> Response {
    if let Some(u) = check_user(&user, &password).await {
        format!("User {} verified\n", u.name).into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            format!("User does not exist/Bad password\n"),
        )
            .into_response()
    }
}

//check if the user is in the database and if the password is correct
//returns None if unsuccessful
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

//provide access or not for the requested path
pub fn verify_access(user: &Option<User>, path: &str) -> bool {
    if let Some(u) = &*user {
        u.admin || path.starts_with("public/") || path.starts_with(&format!("{}/", u.name))   
    } else {
        path.starts_with("public/") 
        
    }
}
