# Overwiew
sylcpn_io is a small project that aim to be a self-hosted cloud to stock and access photos as well as other files remotly. The program is meant to be run on a Linux marchine that use its file system as the database.  
## Features
* Add , Remove  and acess files and directory  
* Authentification to securize private data  
* Public repository to share data with anyone
* Web app to communicate with the server with a nice user interface (comming later)

## TLS
Remplace main.rs with the following :

```rust
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

use axum_server::tls_rustls::RustlsConfig;
use rustls::crypto::CryptoProvider;

#[tokio::main]
async fn main() {

    CryptoProvider::install_default(rustls::crypto::ring::default_provider()).unwrap();
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
            "/rename/usr/:user/psw/:password/to/:name/files/*path",
            post(rename_ressource),
        )
        .layer(DefaultBodyLimit::disable());

    let config = RustlsConfig::from_pem_file("/etc/letsencrypt/live/sylcpn.ddns.net/fullchain.pem", "/etc/letsencrypt/live/sylcpn.ddns.net/privkey.pem",).await.unwrap();

    let addr = local_ip().unwrap();
    let port = 443;
    println!("Server listening on {addr:?}:{port}");
    let full_addr = std::net::SocketAddr::new(addr,port);
    axum_server::bind_rustls(full_addr,config).serve(app.into_make_service()).await.unwrap();



}

```

and add the following dependencies in `Cargo.toml`

```toml 
[dependencies]
axum = { version = "0.7.5", features = ["json", "tokio"] }
axum-server =  { version = "0.7.2" , features = ["tls-rustls-no-provider"] }
local-ip-address = "0.6.1"
rustls = { version = "0.23.25" , features = ["ring"]}
serde = { version = "1.0.203", features = ["derive"] }
serde_json = "1.0.117"
tokio = { version = "1.38.0", features = ["full"] }
```
