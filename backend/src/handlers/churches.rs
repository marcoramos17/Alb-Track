use axum::{
    extract::{Query, State, Multipart},
    http::StatusCode,
    Json,
};
use sqlx::{SqlitePool, Arguments};
use crate::types;

pub async fn create_church(
    State(pool): State<SqlitePool>,
    mut multipart: Multipart,
) -> Result<Json<types::Church>, StatusCode> {

    let mut church_name: Option<String> = None;

    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or("").to_string();
        let value = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;

        match name.as_str() {
            "church_name" => church_name = Some(value),
            _ => {}
        }
    }

    if church_name.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let rec = sqlx::query_as!(
        types::Church,
        r#"
        INSERT INTO church (church_name)
        VALUES (?)
        RETURNING church_id, church_name
        "#,
        church_name
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rec))
}


pub async fn fetch_churches_internal(
    pool: &SqlitePool,
    filter: &types::Church,
) -> Vec<types::Church> {

    let mut sql = "SELECT * FROM church WHERE 1=1".to_string();
    let mut args = sqlx::sqlite::SqliteArguments::default();

    // Filter by ID
    if let Some(id) = filter.church_id {
        sql.push_str(" AND church_id = ?");
        args.add(id);
    }

    // Search by name (LIKE)
    if let Some(name) = &filter.church_name {
        if !name.is_empty() {
            sql.push_str(" AND church_name LIKE ?");
            args.add(format!("%{}%", name));
        }
    }

    sqlx::query_as_with(&sql, args)
        .fetch_all(pool)
        .await
        .unwrap()
}

pub async fn fetch_churches(
    State(pool): State<SqlitePool>,
    Query(filter): Query<types::Church>,
) -> Json<Vec<types::Church>> {
    let churches = fetch_churches_internal(&pool, &filter).await;
    Json(churches)
}
