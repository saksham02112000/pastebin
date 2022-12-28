use axum::routing::get;
use axum::routing::post;
use axum::handler::Handler;
use axum::{
    extract::State,
    body::Body,
    http::Request,
};
use serde::Deserialize;

mod files;

#[derive(Deserialize)]
pub struct Payload {
    pub file_data: String,
    pub extension: String,
}

#[tokio::main]
pub async fn main() {
    // Build our application by creating our router.
    let app = axum::Router::new()
        .fallback(
            fallback
        )
        .route("/",
               post(home_html)
        );

    // Run our application as a hyper server on http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}


/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("No route {}", uri))
}


// /// axum handler for "GET /" which returns a string and causes axum to
// /// immediately respond with status code `200 OK` and with the string.
// pub async fn hello() -> String {
//     "Hello, World!".to_string()
// }


/// axum handler that responds with typical HTML coming from a file.
/// This uses the Rust macro `std::include_str` to include a UTF-8 file
/// path, relative to `main.rs`, as a `&'static str` at compile time.
async fn home_html(
    axum::extract::Json(data): axum::extract::Json<Payload>
) -> String {
    files::create_new_file(data.file_data, data.extension).await;
    // include_str!("./frontend/index.html").into()
    "POST foo".to_string()
}