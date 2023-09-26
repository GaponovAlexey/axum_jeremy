//mod
mod error;
mod web;
mod model;
mod ctx;
mod log;

// use
use std::{ net::SocketAddr };
use ctx::Ctx;
use model::ModelController;
use serde::{ Deserialize, Serialize };
// #![allow(unused)]
use axum::{
    Router,
    routing::{ get, get_service },
    response::{ IntoResponse, Html, Response },
    extract::{ Query, Path },
    middleware,
    Json, http::{Method, Uri},
};
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

pub use error::{ Error, Result };
use uuid::Uuid;

use crate::log::log_request;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets
        ::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        // -- Routes
        .merge(routes())
        .merge(web::routes_login::routes())

        .nest("/api", routes_apis)
        // -- Layer
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(mc.clone(), web::mw_auth::mw_ctx_resolver))
        .layer(CookieManagerLayer::new())
        // -- Smart static routes
        .fallback_service(routes_static());
    // start
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr).serve(routes_all.into_make_service()).await.unwrap();
    Ok(())
}

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response) -> Response {
    println!("Res_Mapper last ");
    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());
    // -- if client error, create new response
    let error_response = client_status_error.as_ref().map(|(status_code, client_err)| {
        let client_error_body =
            json!({
            "error": {
                "type": client_err.as_ref(),
                "req_uuid": uuid.to_string(),
            }
        });
        println!("->> Error : {:?}", client_error_body);
        (*status_code, Json(client_error_body)).into_response()
    });
    // -- Todo:
    let client_err = client_status_error.unzip().1;
    log_request(uuid, req_method, uri,ctx, service_error, client_err).await;

    println!();
    error_response.unwrap_or(res)
}

//
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
//
fn routes() -> Router {
    Router::new().route("/", get(hi)).route("/hi2/:path", get(hi2))
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
