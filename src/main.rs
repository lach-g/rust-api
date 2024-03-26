use axum::extract::{Json, State};
use axum::{routing::get, Router};
use chrono::NaiveDateTime;
use log::info;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::{FromRow, PgPool};
use std::{error::Error, net::SocketAddr, time::Duration};

#[derive(Debug, FromRow, Serialize)]
struct User {
    id: i32,
    username: String,
    email: String,
    created_at: NaiveDateTime,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    dotenv::dotenv().ok();
    let port_str = dotenv::var("API_PORT").unwrap_or("3000".to_string());

    let port: u16 = port_str
        .parse()
        .expect("Failed to parse API_PORT as integer");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let db_url = dotenv::var("DATABASE_URL").expect("Expected DATABASE_URL in the environment");
    info!("Database URL: {db_url}");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_url)
        .await
        .expect("Connection to db failed");

    info!("Serving on: {:?}", addr);

    let app = Router::new()
        .route("/users", get(handle_users))
        .route("/hello", get(handle_hello))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("Created listener. Now serving.");
    axum::serve(listener, app).await.unwrap();

    info!("Server closing");
    Ok(())
}

async fn handle_hello() -> &'static str {
    info!("Entered /hello endpoint function");
    "Hello there"
}

async fn handle_users(State(pool): State<PgPool>) -> Json<Vec<User>> {
    info!("Entered /users endpoint function");

    let query = "SELECT * FROM users";
    let users = sqlx::query_as::<_, User>(query)
        .fetch_all(&pool)
        .await
        .expect("Query to database failed.");
    info!("From callback: Users: {:?}", users);
    Json(users)
}
