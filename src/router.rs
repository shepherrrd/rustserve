use serde_json::json;
use std::collections::HashMap;

pub struct Router {
    routes: HashMap<String, String>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, path: String, response: String) {
        self.routes.insert(path, response);
    }

    pub fn route(&self, path: &str) -> String {
        self.routes.get(path).cloned().unwrap_or_else(|| "HTTP/1.1 404 NOT FOUND\r\n\r\nRoute not found".to_string())
    }
}
pub fn route_request(request: &str) -> String {
    let lines: Vec<&str> = request.lines().collect();
    let request_line = lines[0];
    let request_parts: Vec<&str> = request_line.split_whitespace().collect();
    if request_parts.len() < 2 {
        println!("Bad request: {}", request);
        return "HTTP/1.1 400 BAD REQUEST\r\n\r\nBad Request".to_string();
    }
    let method = request_parts[0];
    let path = request_parts[1];

    match method {
        "GET" => return handle_get(path),
        "POST" => return handle_post(path),
        _ =>  return "HTTP/1.1 405 METHOD NOT ALLOWED\r\n\r\nMethod Not Allowed".to_string(),
    }
}

fn handle_get(path: &str) -> String {
    let response;
    match path {
        "/hello" => {
            let message = json!({
                "message": "Hello from Shepherd!"
            });
            response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}", serde_json::to_string(&message).unwrap());
        },
        _ => {
            let error = json!({
                "error": "Page not found"
            });
            response = format!("HTTP/1.1 404 NOT FOUND\r\nContent-Type: application/json\r\n\r\n{}", serde_json::to_string(&error).unwrap());
        },
    }
    response
}

fn handle_post(path: &str) -> String {
    let response;
    match path {
        "/hello" => {
            let message = json!({
                "message": "Hello from POST!"
            });
            response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}", serde_json::to_string(&message).unwrap());
        },
        _ => {
            let error = json!({
                "error": "Page not found"
            });
            response = format!("HTTP/1.1 404 NOT FOUND\r\nContent-Type: application/json\r\n\r\n{}", serde_json::to_string(&error).unwrap());
        },
    }
    response
}

