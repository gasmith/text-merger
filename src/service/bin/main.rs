//! HTTP server boilerplate.

use std::collections::HashMap;

use axum::{
    Router,
    body::Body,
    extract::Query,
    http::{StatusCode, header::CONTENT_TYPE},
    response::Response,
    routing::get,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:9050")
        .await
        .unwrap();
    let local_addr = listener.local_addr().unwrap();
    println!("Visit http://{local_addr}/?hello=world");
    axum::serve(listener, app).await.unwrap();
}

async fn handler(Query(params): Query<HashMap<String, String>>) -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(format!("your params are {params:?}\n")))
        .unwrap()
}
