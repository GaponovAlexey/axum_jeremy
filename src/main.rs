use std::net::SocketAddr;
use serde::{ Deserialize, Serialize };
// #![allow(unused)]
use axum::{
    Router,
    routing::{ get, get_service },
    response::{ IntoResponse, Html },
    extract::{ Query, Path },
};
use tower_http::services::ServeDir;

pub use self::error::{Error, Result};


//mod
mod error;
mod web;
#[tokio::main]
async fn main() {
    let routes_all = Router::new().merge(routes()).fallback_service(routes_static());
    // start
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr).serve(routes_all.into_make_service()).await.unwrap()
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes() -> Router {
    Router::new().route("/hi", get(hi)).route("/hi2/:path", get(hi2))
}

// handler
#[derive(Serialize, Deserialize, Debug)]
struct HP {
    name: Option<String>,
}
// query
async fn hi(Query(pa): Query<HP>) -> impl IntoResponse {
    let name = pa.name.as_deref().unwrap_or("world");
    Html(format!("da {name}"))
}
// path
async fn hi2(Path(path): Path<String>) -> impl IntoResponse {
    let name = path;
    Html(format!("da {name}"))
}
