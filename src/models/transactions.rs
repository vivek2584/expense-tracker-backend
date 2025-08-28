use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::models::categories::GetUserCategories;

#[derive(Deserialize)]
pub struct TransactionRequest {
    pub category: String,
    pub description: Option<String>,
    pub transaction_date: DateTime<Utc>,
    pub amount: Decimal,
}

#[derive(FromRow)]
pub struct GetCategoryId {
    pub id: Uuid,
}

#[derive(FromRow, Serialize)]
pub struct TransactionInfo {
    pub id: Uuid,
    pub description: Option<String>,
    pub amount: Decimal,
    pub transaction_date: DateTime<Utc>,
    #[sqlx(flatten)]
    #[serde(flatten)]
    pub category_data: GetUserCategories,
}
