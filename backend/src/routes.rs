use axum::{Router, routing::post};
use crate::handlers::users;
use sqlx::SqlitePool;

pub fn routes() -> Router<SqlitePool> {
    Router::new()
        .route("/api/users", post(users::create_user))
}
