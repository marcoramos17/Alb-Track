use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;
use crate::types;

// ENRICHED Implementation
#[derive(Serialize)]
pub struct EnrichedData {
    pub users: Vec<types::User>,
    pub churches: Vec<types::Church>,
    pub albs: Vec<types::Alb>,

    pub alb_church_pairs: Vec<(i64, Option<i64>)>,
}

// USER Implementation
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct User {
    pub user_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub birth_date: Option<String>,
    pub password: Option<String>,
    pub active: Option<bool>,
    pub needs_swap: Option<bool>,
    pub swap_notes: Option<String>,
    pub notes: Option<String>,
}

// ALB Implementation
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Alb {
    pub alb_id: Option<i64>,
    pub alb_code: Option<String>,
    pub alb_size: Option<i64>,
    pub adult_alb: Option<i64>,
    pub has_accessory: Option<i64>,
    pub notes: Option<String>,
}

// CHURCH Implementation
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Church {
    pub church_id: Option<i64>,
    pub church_name: Option<String>,
}

// ALB <-> USER Assignment
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct AlbUserAssignment {
    pub user_id: Option<i64>,
    pub alb_id: Option<i64>,
}

// ALB <-> USER Assignment
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct UserChurchAssignment {
    pub user_id: Option<i64>,
    pub church_id: Option<i64>,
}

// ALB <-> USER Assignment
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ChurchAlbAssignment {
    pub user_id: Option<i64>,
    pub alb_id: Option<i64>,
}