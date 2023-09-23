use crate::{ Error, Result, web };
use axum::{ Json, routing::post, Router };
use serde::Deserialize;
use serde_json::{ Value, json };
use tower_cookies::{ Cookies, Cookie };

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("Handler api login");
    //todo
    if payload.username != "admin" || payload.pwd != "admin" {
        return Err(Error::LoginFail);
    }
    //todo set cookies
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
      "result": {
        "seccess": true
      }
    }));
    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
