pub mod user_routes;

use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::models::user::User;

#[derive(OpenApi)]
#[openapi(
    paths(
        user_routes::get_users
    ),
    components(
        schemas(User)
    ),
    tags(
        (name = "users", description = "User management")
    )
)]
struct ApiDoc;

pub fn all_routes() -> Router {
    Router::new()
        .merge(SwaggerUi::new("/doc").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(user_routes::routes())
        //.merge(other_routes::routes())
}
