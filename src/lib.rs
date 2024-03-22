pub mod identifier;
pub mod header;

use sscanf::sscanf;

use identifier::*;
use header::*;

use openssl::ssl::{SslMethod, SslAcceptor, SslStream, SslFiletype};

use std::{fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};



pub fn handle_client<T: std::io::Read + std::io::Write>(mut stream: T) {


    let mut curent_user : Option<User> = None;
    //check_user("sylane","password",&mut curent_user);
    //dbg!(&curent_user);
    /*let paths = fs::read_dir("./src").unwrap();
    for path in paths {
      println!("Name: {}", path.unwrap().path().display())
  }*/



    loop {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

    let mut header = Header::new();
    process_header(&mut header,&http_request);
    dbg!(&http_request);


    if header.failure{
                break;
        }


    else if header.request_uri.starts_with("/connection_form") {
          if let Ok((name,password)) = sscanf!(header.request_uri,"/connection_form?name={}&password={}",String,String) {

            check_user(&name,&password,&mut curent_user);
            if let Some(ref usr) = curent_user {

              let status_line = "HTTP/1.1 200 OK";
              //let contents = fs::read_to_string("hello.html").unwrap();
              let contents = format!("<!DOCTYPE html>
              <html lang=\"en\">
                <head>
                  <meta charset=\"utf-8\">
                  <title>Hello!</title>
                </head>
                <body>
                  <h1>Hello!</h1>
                  <p>Hi from Rust</p>
                  <p>You are {} </p>
                </body>
              </html>", usr.name );
              let length = contents.len();
              let response =format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
          
              stream.write_all(response.as_bytes()).unwrap();
          

            }

            else {
              let status_line = "HTTP/1.1 200 OK";
    //let contents = fs::read_to_string("hello.html").unwrap();
    let contents = format!("<!DOCTYPE html>
    <html lang=\"en\">
    <head>
        <meta charset=\"UTF-8\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
        <title>Login</title>
    </head>
    <body>
        <h1>Connection</h1>
        <p> Not in dataBase </p>
        <form action=\"/connection_form\">
        <label for=\"name\">First name:</label><br>
        <input type=\"text\" id=\"name\" name=\"name\" value=\"John\"><br>
        <label for=\"password\">Last name:</label><br>
        <input type=\"text\" id=\"password\" name=\"password\" value=\"Doe\"><br><br>
        <input type=\"submit\" value=\"Submit\">
          </form> 
        
    </body>
    </html>");
    let length = contents.len();
    let response =format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
            }

          }
    }

    else if header.request_method.starts_with("GET") {
            handle_get(&mut stream, &header,&curent_user);
        }

    }
    
}

fn handle_get<T : std::io::Read + std::io::Write>(stream : &mut T ,request : &Header,user : &Option<User> ) {

  if request.request_uri == "/test"{

    let status_line = "HTTP/1.1 200 OK";
    //let contents = fs::read_to_string("hello.html").unwrap();
    let contents = format!("<!DOCTYPE html>
    <html lang=\"en\">
      <head>
        <meta charset=\"utf-8\">
        <title>Hello!</title>
      </head>
      <body>
        <h1>Hello!</h1>
        <p>Hi from Rust</p>
        <p>You are {} </p>
      </body>
    </html>", if let Some(usr) = user {usr.name.clone()} else {String::from("Anonymous")} );
    let length = contents.len();
    let response =format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

  }

  if request.request_uri == "/"{

    let status_line = "HTTP/1.1 200 OK";
    //let contents = fs::read_to_string("hello.html").unwrap();
    let contents = format!("<!DOCTYPE html>
    <html lang=\"en\">
    <head>
        <meta charset=\"UTF-8\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
        <title>Login</title>
    </head>
    <body>
        <h1>Connection</h1>
        <form action=\"/connection_form\">
        <label for=\"name\">First name:</label><br>
        <input type=\"text\" id=\"name\" name=\"name\" value=\"John\"><br>
        <label for=\"password\">Last name:</label><br>
        <input type=\"text\" id=\"password\" name=\"password\" value=\"Doe\"><br><br>
        <input type=\"submit\" value=\"Submit\">
          </form> 
        
    </body>
    </html>");
    let length = contents.len();
    let response =format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

  }

  

}