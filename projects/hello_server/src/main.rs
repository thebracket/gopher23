use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Bind the default route to the function `say_hello_text`
    let app = Router::new().route("/", get(say_hello_text));

    // Listen on localhost, port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Return a static string
async fn say_hello_text() -> &'static str {
    "Hello, world!"
}