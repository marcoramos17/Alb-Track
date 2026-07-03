use axum::{
    extract::{Multipart, State, Query},
    Json,
    http::StatusCode,
};
use sqlx::{SqlitePool, Arguments};
use tokio::fs;
use crate::types;

use crate::handlers::relations;

pub async fn create_user(
    State(pool): State<SqlitePool>,
    mut multipart: Multipart,
) -> Result<String, StatusCode> {

    let mut first_name = String::new();
    let mut last_name = String::new();
    let mut phone = String::new();
    let mut birth_date = String::new();
    let mut password = String::new();
    let mut photo_bytes: Option<Vec<u8>> = None;
    let mut photo_filename: Option<String> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "first_name" => first_name = field.text().await.unwrap(),
            "last_name" => last_name = field.text().await.unwrap(),
            "phone" => phone = field.text().await.unwrap(),
            "birth_date" => birth_date = field.text().await.unwrap(),
            "password" => password = field.text().await.unwrap(),
            "photo" => {
                photo_filename = Some(field.file_name().unwrap().to_string());
                photo_bytes = Some(field.bytes().await.unwrap().to_vec());
            }
            _ => {}
        }
    }

    // Insert user and get ID
    let rec = sqlx::query!(
        "INSERT INTO users (first_name, last_name, phone, birth_date, password)
         VALUES (?, ?, ?, ?, ?)",
        first_name,
        last_name,
        phone,
        birth_date,
        password
    )
    .execute(&pool)
    .await
    .unwrap();

    let user_id = rec.last_insert_rowid();

    // Save photo
    if let Some(bytes) = photo_bytes {
        let ext = photo_filename
            .as_ref()
            .and_then(|f| f.split('.').last())
            .unwrap_or("jpg");

        let path = format!("backend/data/photos/{}.{}", user_id, ext);

        fs::write(path, bytes).await.unwrap();
    }

    Ok(format!("User {} created", user_id))
}

pub async fn fetch_users_internal(
    pool: &SqlitePool,
    filter: &types::User,
) -> Vec<types::User> {

    let mut sql = "SELECT * FROM users WHERE 1=1".to_string();
    let mut args = sqlx::sqlite::SqliteArguments::default();

    if let Some(id) = filter.user_id {
        sql.push_str(" AND user_id = ?");
        args.add(id);
    }

    
    
    if let Some(search) = &filter.first_name {
        if !search.is_empty() {
            if search.contains(' ') {
                // Full name search
                let parts: Vec<&str> = search.split_whitespace().collect();
                if parts.len() >= 2 {
                    let first = parts[0];
                    let last = parts[1];

                    sql.push_str(" AND first_name LIKE ?");
                    args.add(format!("%{}%", first));

                    sql.push_str(" AND last_name LIKE ?");
                    args.add(format!("%{}%", last));
                }
            } else {
                // Single term search (OR)
                sql.push_str(" AND (first_name LIKE ? OR last_name LIKE ?)");
                let like = format!("%{}%", search);
                args.add(like.clone());
                args.add(like);
            }
        }
    }




    if let Some(active) = filter.active {
        sql.push_str(" AND active = ?");
        args.add(active);
    }

    // ignore fields you don't want to filter by

    sqlx::query_as_with(&sql, args)
        .fetch_all(pool)
        .await
        .unwrap()

}


pub async fn fetch_users(
    State(pool): State<SqlitePool>,
    Query(filter): Query<types::User>,
) -> Json<types::EnrichedData> {

    let users = fetch_users_internal(&pool, &filter).await;

    let mut churches = Vec::new();
    let mut albs = Vec::new();
    let mut alb_church_pairs = Vec::new();

    for user in &users {
        let uid = user.user_id.unwrap();

        //
        // USER ↔ CHURCH
        //
        let church_links = relations::user_church_relation(&pool, Some(uid), None).await;

        for link in church_links {
            if let Some(cid) = link.church_id {
                let church = sqlx::query_as!(
                    types::Church,
                    "SELECT * FROM church WHERE church_id = ?",
                    cid
                )
                .fetch_one(&pool)
                .await
                .unwrap();

                churches.push(church);

                // record mapping: user → church
                alb_church_pairs.push((uid, 0, cid)); // alb_id = 0 means "no alb"
            }
        }

        //
        // USER ↔ ALB
        //
        let alb_links = relations::user_alb_relation(&pool, Some(uid), None).await;

        for link in alb_links {
            if let Some(aid) = link.alb_id {
                let alb = sqlx::query_as!(
                    types::Alb,
                    "SELECT * FROM albs WHERE alb_id = ?",
                    aid
                )
                .fetch_one(&pool)
                .await
                .unwrap();

                albs.push(alb);

                //
                // ALB ↔ CHURCH
                //
                let alb_church = relations::church_alb_relation(&pool, None, Some(aid)).await;
                let church_id = alb_church.first().and_then(|c| c.church_id);

                // record mapping: user → alb → church
                alb_church_pairs.push((uid, aid, church_id.unwrap_or(0)));
            }
        }
    }

    Json(types::EnrichedData {
        users,
        churches,
        albs,
        alb_church_pairs,
    })
}
