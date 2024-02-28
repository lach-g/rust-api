use axum::{routing::get, Router};
use env_logger;
use log::info;

#[tokio::main]
async fn main() {
    env_logger::init();
    let address = "0.0.0.0:3000";

    info!("Configuring to run on {address}");

    // build our application with a single route
    let app = Router::new()
        .route("/hello", get(handle_hello))
        .route("/about", get(handle_about))
        .route("/time", get(handle_time));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    info!("Serving on {address}");
    axum::serve(listener, app).await.unwrap();
}

async fn handle_hello() -> &'static str {
    "Hello there"
}

async fn handle_about() -> &'static str {
    "This is a server written in Rust!"
}

async fn handle_time() -> String {
    format!("Current Local Time: {:?}", chrono::offset::Local::now())
}

