
pub fn route_request(request: &str) -> String {
    let lines: Vec<&str> = request.lines().collect();
    let request_line = lines[0];
    let request_parts: Vec<&str> = request_line.split_whitespace().collect();
    let method = request_parts[0];
    let path = request_parts[1];

    match method {
        "GET" => handle_get(path),
        "POST" => handle_post(path),
        _ => "HTTP/1.1 405 METHOD NOT ALLOWED\r\n\r\nMethod Not Allowed".to_string(),
    }
}

fn handle_get(path: &str) -> String {
    match path {
        "/hello" => "HTTP/1.1 200 OK\r\n\r\nHello from GET!".to_string(),
        _ => "HTTP/1.1 404 NOT FOUND\r\n\r\nPage not found".to_string(),
    }
}

fn handle_post(path: &str) -> String {
    match path {
        "/hello" => "HTTP/1.1 200 OK\r\n\r\nHello from POST!".to_string(),
        _ => "HTTP/1.1 404 NOT FOUND\r\n\r\nPage not found".to_string(),
    }
}
