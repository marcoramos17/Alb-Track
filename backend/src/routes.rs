use axum::{Router, routing::{get, post}};
use crate::handlers::{users,albs, churches};
use sqlx::SqlitePool;

pub fn routes() -> Router<SqlitePool> {
    Router::new()
        .route("/api/users", post(users::create_user)
                                                .get(users::fetch_users)
        )
        .route("/api/albs", post(albs::create_alb)
                                                .get(albs::fetch_albs)
        )
        .route("/api/church", post(churches::create_church)
                                                .get(churches::fetch_churches)
        )

}
