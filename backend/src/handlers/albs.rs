use axum::{
    extract::{Query, State, Multipart},
    http::StatusCode,
    Json,
};
use sqlx::{SqlitePool, Arguments};
use crate::types;

pub async fn create_alb(
    State(pool): State<SqlitePool>,
    mut multipart: Multipart,
) -> Result<Json<types::Alb>, StatusCode> {

    let mut alb_code: Option<String> = None;
    let mut alb_size: Option<i64> = None;
    let mut adult_alb: Option<i64> = None;
    let mut has_accessory: Option<i64> = None;
    let mut notes: Option<String> = None;

    // Extract fields from multipart
    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or("").to_string();
        let value = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;

        match name.as_str() {
            "alb_code" => alb_code = Some(value),
            "alb_size" => alb_size = value.parse::<i64>().ok(),
            "adult_alb" => adult_alb = value.parse::<i64>().ok(),
            "has_accessory" => has_accessory = value.parse::<i64>().ok(),
            "notes" => notes = Some(value),
            _ => {}
        }
    }

    // Basic validation
    if alb_code.is_none() || alb_size.is_none() || adult_alb.is_none() || has_accessory.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Insert into database
    let rec = sqlx::query_as!(
        types::Alb,
        r#"
        INSERT INTO albs (alb_code, alb_size, adult_alb, has_accessory, notes)
        VALUES (?, ?, ?, ?, ?)
        RETURNING alb_id, alb_code, alb_size, adult_alb, has_accessory, notes
        "#,
        alb_code,
        alb_size,
        adult_alb,
        has_accessory,
        notes
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rec))
}

pub async fn fetch_albs_internal(
    pool: &SqlitePool,
    filter: &types::Alb,
) -> Vec<types::Alb> {

    let mut sql = "SELECT * FROM albs WHERE 1=1".to_string();
    let mut args = sqlx::sqlite::SqliteArguments::default();

    // Search by alb_code OR alb_size
    if let Some(search) = &filter.alb_code {
        if !search.is_empty() {
            sql.push_str(" AND (alb_code LIKE ? OR alb_size LIKE ?)");
            let like = format!("%{}%", search);
            args.add(like.clone());
            args.add(like);
        }
    }

    // Filter adult/child
    if let Some(adult) = filter.adult_alb {
        sql.push_str(" AND adult_alb = ?");
        args.add(adult);
    }

    // Filter accessory
    if let Some(acc) = filter.has_accessory {
        sql.push_str(" AND has_accessory = ?");
        args.add(acc);
    }

    sqlx::query_as_with(&sql, args)
        .fetch_all(pool)
        .await
        .unwrap()
}

pub async fn fetch_albs(
    State(pool): State<SqlitePool>,
    Query(filter): Query<types::Alb>,
) -> Json<Vec<types::Alb>> {
    let albs = fetch_albs_internal(&pool, &filter).await;
    Json(albs)
}
