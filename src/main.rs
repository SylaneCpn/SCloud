mod assets;
mod auth;
mod index;
mod manip;
mod reader;
mod ressource;
mod utils;
mod writer;

use axum::{
    routing::{delete, get, post},
    Router,
    extract::DefaultBodyLimit,
};

use assets::assets;
use auth::verify_user;
use index::root;
use manip::create_file;
use manip::create_dir;
use manip::remove_ressource;
use ressource::files;
use ressource::main_repo;

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
            delete(remove_ressource)
        )
        .route("/adddir/usr/:user/psw/:password/files/*path",
            post(create_dir))
        .route(
            "/addfile/usr/:user/psw/:password/files/*path",
            post(create_file)
        ).layer(DefaultBodyLimit::disable());

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
