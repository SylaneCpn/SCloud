mod assets;
mod index;
mod raw;
mod response_provider;

use axum::{routing::get, Router};

use assets::assets;
use index::root;
use raw::raw;

use local_ip_address::local_ip;
use tokio;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/assets/*path", get(assets))
        .route("/raw/*path", get(raw));

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
