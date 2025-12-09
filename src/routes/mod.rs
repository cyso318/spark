pub mod user_routes;

use axum::Router;

pub fn all_routes() -> Router {
    Router::new()
        .merge(user_routes::routes())
        //.merge(post_routes::routes())
}