use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslStream};
use std::sync::Arc;
use std::thread;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use sylcpn_io::*;

use local_ip_address::local_ip;

fn main() {
    let my_local_ip = local_ip().unwrap().to_string();
    //dbg!(&my_local_ip);

    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    acceptor
        .set_private_key_file(format!("{}-key.pem", &my_local_ip), SslFiletype::PEM)
        .unwrap();
    acceptor
        .set_certificate_chain_file(format!("{}.pem", &my_local_ip))
        .unwrap();
    acceptor.check_private_key().unwrap();
    let acceptor = Arc::new(acceptor.build());

    let https_listener = TcpListener::bind(format!("{}:443", &my_local_ip)).unwrap();

    println!(
        "Server Lauched on IP : {} , waiting for connections...",
        &my_local_ip
    );

    for stream in https_listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("port 443 , stream : {:?}", &stream);
                let acceptor = acceptor.clone();
                thread::spawn(move || match acceptor.accept(stream) {
                    Ok(s) => {
                        handle_client(s);
                    }
                    Err(e) => {
                        println!("Error {:?} occured", e);
                    }
                });
            }
            Err(e) => {
                println!("Error {:?} occured", e);
            }
        }
    }
}
