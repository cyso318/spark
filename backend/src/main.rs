use std::{env, error::Error};

use axum::Router;
use clorinde::{
    deadpool_postgres::{Config, Pool, Runtime},
    tokio_postgres::NoTls,
};

mod api_types;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = app(None);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    return Ok(());
}

fn database_pool(mut database_url: Option<String>) -> Pool {
    if database_url.is_none() {
        database_url =
            Some(env::var("DATABASE_URL").expect("DATABASE_URL should be set, but isn't."));
    }

    let mut cfg = Config::new();
    cfg.url = database_url;

    return cfg
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("Couldn't create database pool.");
}

fn app(database_url: Option<String>) -> Router {
    return routes::all_routes(database_pool(database_url));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_connection() {
        let database_test_url = Some(
            env::var("DATABASE_TEST_URL").expect("DATABASE_TEST_URL should be set, but isn't."),
        );
        let pool = database_pool(database_test_url);

        let client = pool
            .get()
            .await
            .expect("Couldn't get client from database pool.");
        let client_conn_check = client.check_connection().await;
        assert!(!client_conn_check.is_err());
    }
}
