use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::{Outcome};
use serde::{Deserialize, Serialize};

extern crate base64;
const TOKEN_PREFIX: &str = "Basic ";

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth;

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Auth, Self::Error> {
        let secrt: String = String::from("admin:password");
        if let Some(auth) = extract_auth_from_request(request, &secrt.into_bytes()) {
            Outcome::Success(auth)
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}

fn extract_auth_from_request(request: &Request, secret: &[u8]) -> Option<Auth> {
    request
        .headers()
        .get_one("authorization")
        .and_then(extract_token_from_header)
        .and_then(|token| decode_token(token, secret))
}

fn extract_token_from_header(header: &str) -> Option<&str> {
    if header.starts_with(TOKEN_PREFIX) {
        Some(&header[TOKEN_PREFIX.len()..])
    } else {
        None
    }
}

/// Decode token into `Auth` struct. If any error is encountered, log it
/// an return None.
fn decode_token(token: &str, secret: &[u8]) -> Option<Auth> {
    use base64::{decode};
    let decoded = decode(token)
        .map_err(|err| {
            eprintln!("Auth decode error: {:?} token {}", err, token);
        })
        .ok()
        .unwrap();
    if decoded == secret {
        return Some(Auth{});
    } else {
        return None;
    }
}