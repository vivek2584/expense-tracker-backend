use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

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
