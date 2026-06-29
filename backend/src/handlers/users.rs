use axum::{Json, extract::State};
use sqlx::SqlitePool;
use crate::types::CreateUserRequest;


pub async fn create_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateUserRequest>,
) -> &'static str {
    sqlx::query!(
        "INSERT INTO users (first_name, last_name, phone, birth_date, password)
         VALUES (?, ?, ?, ?, ?)",
        payload.first_name,
        payload.last_name,
        payload.phone,
        payload.birth_date,
        payload.password
    )
    .execute(&pool)
    .await
    .expect("Failed to insert user");

    "User created"
}
