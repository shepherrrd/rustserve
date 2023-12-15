mod router;

use router::Router;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn handle_connection(mut stream: TcpStream, router: &Router) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request_line = String::from_utf8_lossy(&buffer[..]).lines().next().unwrap_or("");
    let path = request_line.split_whitespace().nth(1).unwrap_or("");

    let response = router.route(path);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let mut router = Router::new();
    router.add_route("/hello/v1".to_string(), "HTTP/1.1 200 OK\r\n\r\nHello, V1!".to_string());

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &router);
    }
}
fn sample(){
    
}