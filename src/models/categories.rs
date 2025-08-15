use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateCategoryDetails {
    pub name: String,
    pub category_type: CategoryType,
    #[serde(default)]
    pub is_savings: bool,
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "category_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum CategoryType {
    Expense,
    Income,
}

#[derive(FromRow, Serialize)]
pub struct GetUserCategories {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    #[sqlx(rename = "type")]
    pub category_type: CategoryType,
    pub is_savings: bool,
}
