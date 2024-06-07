
mod index;
mod assets;
mod raw;
mod response_provider;

use axum::{routing::get, Router};

use index::root;
use assets::assets;
use raw::raw;

use tokio;

#[tokio::main]
async fn main() {
    
    let app = Router::new()
        .route("/", get(root))
        .route("/assets/*path",get(assets))
        .route("/raw/*path",get(raw));

    let addr = "172.22.27.170:3000";
    println!("Server listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
