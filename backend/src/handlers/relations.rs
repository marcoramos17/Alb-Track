use sqlx::{SqlitePool, Arguments};
use crate::types;

pub async fn user_alb_relation(
    pool: &SqlitePool,
    user_id: Option<i64>,
    alb_id: Option<i64>,
) -> Vec<types::AlbUserAssignment> {

    let mut sql = "SELECT user_id, alb_id FROM user_alb WHERE 1=1".to_string();
    let mut args = sqlx::sqlite::SqliteArguments::default();

    if let Some(uid) = user_id {
        sql.push_str(" AND user_id = ?");
        args.add(uid);
    }

    if let Some(aid) = alb_id {
        sql.push_str(" AND alb_id = ?");
        args.add(aid);
    }

    sqlx::query_as_with(&sql, args)
        .fetch_all(pool)
        .await
        .unwrap()
}


pub async fn user_church_relation(
    pool: &SqlitePool,
    user_id: Option<i64>,
    church_id: Option<i64>,
) -> Vec<types::UserChurchAssignment> {

    let mut sql = "SELECT user_id, church_id FROM user_church WHERE 1=1".to_string();
    let mut args = sqlx::sqlite::SqliteArguments::default();

    if let Some(uid) = user_id {
        sql.push_str(" AND user_id = ?");
        args.add(uid);
    }

    if let Some(cid) = church_id {
        sql.push_str(" AND church_id = ?");
        args.add(cid);
    }

    sqlx::query_as_with(&sql, args)
        .fetch_all(pool)
        .await
        .unwrap()
}


pub async fn church_alb_relation(
    pool: &SqlitePool,
    church_id: Option<i64>,
    alb_id: Option<i64>,
) -> Vec<types::ChurchAlbAssignment> {

    let mut sql = "SELECT church_id, alb_id FROM church_alb WHERE 1=1".to_string();
    let mut args = sqlx::sqlite::SqliteArguments::default();

    if let Some(cid) = church_id {
        sql.push_str(" AND church_id = ?");
        args.add(cid);
    }

    if let Some(aid) = alb_id {
        sql.push_str(" AND alb_id = ?");
        args.add(aid);
    }

    sqlx::query_as_with(&sql, args)
        .fetch_all(pool)
        .await
        .unwrap()
}
