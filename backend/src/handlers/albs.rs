use axum::{
    extract::{Query, State, Multipart},
    http::StatusCode,
    Json,
};
use sqlx::{SqlitePool, Arguments};
use crate::types;
use chrono;

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

async fn user_alb_link(
    pool: &SqlitePool,
    user_id: Option<i64>,
    alb_id: Option<i64>,
) -> Result<(), StatusCode> {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    sqlx::query!(
        r#"
        INSERT INTO user_alb (user_id, alb_id, date)
        VALUES (?, ?, ?)
        "#,
        user_id,
        alb_id,
        today
    )
    .execute(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

pub async fn assign_alb(
    State(pool): State<SqlitePool>,
    Json(assign): Json<types::AlbUserAssignment>,
) -> Result<StatusCode, StatusCode> {
    user_alb_link(&pool, assign.user_id, assign.alb_id).await?;
    Ok(StatusCode::OK)
}

pub async fn unassign_alb(
    State(pool): State<SqlitePool>,
    Json(assign): Json<types::AlbUserAssignment>,
) -> Result<StatusCode, StatusCode> {
    let user_id = assign.user_id;
    let alb_id = assign.alb_id;

    // user -> NULL alb
    user_alb_link(&pool, user_id, None).await?;
    // NULL -> alb
    user_alb_link(&pool, None, alb_id).await?;

    Ok(StatusCode::OK)
}

/*
pub async fn swap_alb(
    State(pool): State<SqlitePool>,
    Json(assign): Json<types::AlbAssignment>, // user_id + new alb_id
) -> Result<StatusCode, StatusCode> {
    let user_id = assign.user_id;
    let new_alb_id = assign.alb_id;

    // 1. Find current alb for this user (old alb)
    let row = sqlx::query!(
        r#"
        SELECT alb_id
        FROM user_alb
        WHERE user_id = ?
        ORDER BY date DESC
        LIMIT 1
        "#,
        user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let old_alb_id = row.and_then(|r| r.alb_id);

    // 2. Assign new alb to user
    insert_user_alb_event(&pool, user_id, new_alb_id).await?;

    // 3. If there was an old alb, mark it as unowned
    if let Some(old_id) = old_alb_id {
        insert_user_alb_event(&pool, None, Some(old_id)).await?;
    }

    Ok(StatusCode::OK)
}
*/