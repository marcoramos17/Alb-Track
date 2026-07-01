use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

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

pub struct AlbAssignment {
    pub user_id: Option<i64>,
    pub alb_id: Option<i64>,
}