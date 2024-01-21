#![allow(unused)]

use axum::{
    Router, 
    response::{IntoResponse, Html, Response},
    routing::{get, get_service},
    extract::Query,
    extract::Path,
    middleware,
};
use std::net::SocketAddr;
use serde::Deserialize;
use tower_http::services::ServeDir;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use tokio::net::TcpListener;

pub use self::error::{Error, Result};

mod error;
mod web;


#[tokio::main]
async fn main() {
    let routes_all = Router::new()
    .merge(routes_hello())
    .merge(web::routes_login::routes())
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new())
    .fallback_service(routes_static());

    // let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await.unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}


// e.g. `/hello?name=Akan`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!!");
    Html(format!("Hello, <strong>>{name}</strong>"))
}

// e.g. `/hello2/Akan`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {name:?}", "HANDLER");

    Html(format!("Hello2, <strong>>{name}</strong>"))
}