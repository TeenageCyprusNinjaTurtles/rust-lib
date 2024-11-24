use std::io::Read;

use base64::Engine;
use rouille::{Request, Response};
use serde::Serialize;

pub fn request_to_bytes(request: &Request) -> Vec<u8> {
    let mut buffer = Vec::new();
    request.data().unwrap().read_to_end(&mut buffer).unwrap();
    buffer
}

pub fn response_to_bytes(response: Response) -> Vec<u8> {
    let mut buffer = Vec::new();
    response.data.into_reader_and_size().0.read_to_end(&mut buffer).unwrap();
    buffer
}

pub fn get_user_level(request: &Request) -> i32 {
    let level: i32 = get_header_value(request, "X-User-Level").unwrap_or("0".to_owned()).parse().unwrap_or(0);
    level
}

pub fn get_header_value(request: &Request, header: &str) -> Option<String> {
    let mut headers = request.headers();
    let value = headers.find(|&k| k.0 == header);
    if value.is_none() {
        return None;
    }
    Some(value.unwrap().1.to_string())
}

pub fn generate_token() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let token: String = (0..32)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect();
    token
}

pub fn from_b64(input: &str) -> String {
    let decoded = base64::prelude::BASE64_STANDARD.decode(input.as_bytes()).unwrap();
    String::from_utf8(decoded).unwrap()
}

pub fn to_b64(input: &str) -> String {
    base64::prelude::BASE64_STANDARD.encode(input.as_bytes())
}

#[derive(Serialize)]
struct ResultResponse {
    result: String
}

pub fn return_result(result: String) -> Response {
    Response::json(&ResultResponse { result })
}
