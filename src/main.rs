mod assets;
mod auth;
mod index;
mod manip;
mod raw;
mod response_provider;

use axum::{
    routing::{delete, get, post},
    Router,
};

use assets::assets;
use auth::verify_user;
use index::root;
use manip::create_file;
use manip::remove_ressource;
use raw::files;
use raw::main_repo;

use local_ip_address::local_ip;
use tokio;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/assets/*path", get(assets))
        .route("/usr/:user/psw/:password/files/*path", get(files))
        .route("/usr/:user/psw/:password/files/", get(main_repo))
        .route("/usr/:user/psw/:password/", get(verify_user))
        .route(
            "/rm/usr/:user/psw/:password/files/*path",
            delete(remove_ressource),
        )
        .route(
            "/addf/usr/:user/psw/:password/files/*path",
            post(create_file),
        );

    let addr = local_ip().unwrap().to_string();
    let port = 3000;
    println!("Server listening on {addr}:{port}");
    let listener = tokio::net::TcpListener::bind(format!("{addr}:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
