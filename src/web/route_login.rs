use crate::{Error, Result};
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

// can only have one body extractor per route
// and has to be the last argument
// Error impl IntoResponse that is why we can use Result
async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    // TODO: DB and Auth Login

    if payload.username != "movindu" || payload.password != "mypass" {
        return Err(Error::LoginFail);
    }

    // FIXME: Real Auth Token generation adn signature
    let cookie = Cookie::build(("auth-token", "user-1.exp.sign"))
        .path("/api")
        .build();
    // cookies.add(Cookie::new("auth-token", "user-1.exp.sign"));
    cookies.add(cookie);

    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));

    Ok(body)
}
