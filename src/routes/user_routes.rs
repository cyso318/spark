use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new()
        .route("/users", get(get_users))
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "Say 'List of users'", body = String)
    )
)]
async fn get_users() -> &'static str {
    //TODO db request to get users and return them
    return "List of users";
}
