use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow};
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct PersonModel {
    pub id: Uuid,
    pub username: String,
    pub age: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
