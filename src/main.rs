use axum::extract::{Json, Query, State};
use axum::{routing::get, Router};
use chrono::NaiveDateTime;
use log::info;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::{error::Error, net::SocketAddr, time::Duration};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
struct User {
    id: i32,
    username: String,
    email: String,
    created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
struct UserIdParam {
    id: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct UserCreateParams {
    username: String,
    email: String,
    created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
struct UserUpdateParams {
    id: i32,
    username: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let port: u16 = dotenv::var("API_PORT")
        .unwrap_or("3000".to_string())
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

    let router = Router::new()
        .route("/", get(handle_root))
        .route(
            "/user",
            get(read_users)
                .post(create_user)
                .delete(delete_user)
                .put(update_user),
        )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("Created listener. Now serving.");
    axum::serve(listener, router).await.unwrap();

    info!("Server closing");
    Ok(())
}

async fn handle_root() -> &'static str {
    info!("--> / Endpoint Handler: handle_root()");
    "Hello there"
}

// User CREATE
async fn create_user(State(pool): State<PgPool>, user_body: Json<UserCreateParams>) -> Json<User> {
    info!("--> /USER CREATE");

    let creating = user_body.0;

    let new_user: User = match creating.created_at {
        Some(created_at) => {
            let query = "INSERT INTO users (username, email, created_at) VALUES ($1, $2, $3) RETURNING id, username, email, created_at";
            sqlx::query_as(query)
                .bind(creating.username)
                .bind(creating.email)
                .bind(created_at)
                .fetch_one(&pool)
                .await
                .expect("Fail: Database User INSERT query_as method.")
        }
        None => {
            let query = "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id, username, email, created_at";
            sqlx::query_as(query)
                .bind(creating.username)
                .bind(creating.email)
                .fetch_one(&pool)
                .await
                .expect("Fail: Database User INSERT query_as method.")
        }
    };

    info!("Created user: {:?}.", new_user);
    Json(new_user)
}

// User READ
async fn read_users(
    State(pool): State<PgPool>,
    Query(user_params): Query<UserIdParam>,
) -> Json<Vec<User>> {
    info!("--> /USER READ");

    let users = match user_params.id {
        Some(id) => {
            let query = "SELECT * FROM users WHERE id = $1";
            sqlx::query_as::<_, User>(query)
                .bind(id)
                .fetch_all(&pool)
                .await
                .expect("Query to database failed.")
        }
        None => {
            let query = "SELECT * FROM users";
            sqlx::query_as::<_, User>(query)
                .fetch_all(&pool)
                .await
                .expect("Query to database failed.")
        }
    };

    info!("From callback: Users: {:?}", users);
    Json(users)
}

// User UPDATE
// TODO: Should I change this to optional parameter updates
async fn update_user(State(pool): State<PgPool>, user_body: Json<UserUpdateParams>) -> Json<User> {
    info!("--> /USER UPDATE");

    let user = user_body.0;

    let query = "UPDATE users SET username = $1, email = $2 WHERE id = $3 RETURNING id, username, email, created_at";
    let updated_user = sqlx::query_as(query)
        .bind(user.username)
        .bind(user.email)
        .bind(user.id)
        .fetch_one(&pool)
        .await
        .expect("Fail: Database User UPDATE");

    Json(updated_user)
}

// User DELETE
async fn delete_user(
    State(pool): State<PgPool>,
    Query(user_params): Query<UserIdParam>,
) -> Json<User> {
    info!("--> /USER DELETE");

    let id = user_params.id.expect("Fail: User ID not present");

    let query = "DELETE FROM users WHERE id = $1 RETURNING id, username, email, created_at";

    let user: User = sqlx::query_as(query)
        .bind(id)
        .fetch_one(&pool)
        .await
        .expect("Fail: Database User DELETE.");

    Json(user)
}
