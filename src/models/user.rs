use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(ToSchema, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}
