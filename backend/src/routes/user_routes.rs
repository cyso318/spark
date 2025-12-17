use axum::{Json, Router, extract::State, http::StatusCode, routing::{get, post}};
use sqlx::{PgPool, Postgres};

use crate::models::user::User;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/users", get(get_users))
        .route("/user", post(post_user))
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = StatusCode::OK, description = "Return a list of all users", body = Vec<User>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Returns the error message", body = String)
    )
)]
async fn get_users(State(pool): State<PgPool>) -> (StatusCode, Result<Json<Vec<User>>, String>) {
    let query = sqlx::query_as::<Postgres, User>("SELECT id, username FROM users");

    let result = query.fetch_all(&pool).await;
    if result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Err(result.unwrap_err().to_string()));
    }
    return (StatusCode::OK, Ok(Json(result.unwrap())));
}

#[utoipa::path(
    post,
    path = "/user",
    request_body(
        content = String,
        description = "Username of the user to be created"
    ),
    responses(
        (status = StatusCode::CREATED, description = "Successfully created user with username", body = User),
        (status = StatusCode::CONFLICT, description = "User already exists", body = String),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Returns the error message", body = String)
    )
)]
async fn post_user(State(pool): State<PgPool>, username: String) -> (StatusCode, Result<Json<User>, String>) {
    let query = sqlx::query_as::<Postgres, User>("INSERT INTO users (username) VALUES ($1) RETURNING id, username").bind(username);

    let result = query.fetch_one(&pool).await;
    match result {
        Ok(row) => {
            let created_user: User = User {id: row.id, username: row.username};
            return (StatusCode::CREATED, Ok(Json(created_user)));
        }
        Err(e) => {
            if let Some(some_eb_error) = e.as_database_error() {
                // Check for PostgreSQL unique violation error code
                if let Some(code) = some_eb_error.code() {
                    if code == "23505" { // ERROR: duplicate key value violates unique constraint "..."
                        return (StatusCode::CONFLICT, Err("User already exists".to_string()));
                    }
                }
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, Err(e.to_string()));
        }
    }
}