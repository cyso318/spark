use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/users", get(get_users))
}

async fn get_users() -> &'static str {

    //TODO db request to get users and return them
    return "List of users";
}