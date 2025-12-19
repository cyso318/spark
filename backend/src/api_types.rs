use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, ToSchema, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}
