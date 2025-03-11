mod assets;
mod auth;
mod index;
mod manip;
mod reader;
mod ressource;
mod utils;
mod writer;

use axum::{
    extract::DefaultBodyLimit,
    routing::{delete, get, post},
    Router,
};

use assets::assets;
use auth::verify_user;
use index::root;
use manip::create_dir;
use manip::create_file;
use manip::remove_ressource;
use manip::rename_ressource;
use ressource::files;
use ressource::main_repo;

use local_ip_address::local_ip;
use tokio;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/*path", get(assets))
        .route("/usr/:user/psw/:password/files/*path", get(files))
        .route("/usr/:user/psw/:password/files/", get(main_repo))
        .route("/usr/:user/psw/:password/", get(verify_user))
        .route(
            "/rm/usr/:user/psw/:password/files/*path",
            delete(remove_ressource),
        )
        .route(
            "/adddir/usr/:user/psw/:password/files/*path",
            post(create_dir),
        )
        .route(
            "/addfile/usr/:user/psw/:password/files/*path",
            post(create_file),
        )
        .route(
            "/rename/usr/:user/psw/:password/to/:name/files/*path"
        , post(rename_ressource),
        )
        .layer(DefaultBodyLimit::disable());

    let addr = local_ip().unwrap().to_string();
    // let addr = "127.0.0.1";
    let port = 8000;
    println!("Server listening on {addr}:{port}");
    let listener = tokio::net::TcpListener::bind(format!("{addr}:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
