use axum::{extract::State, http::StatusCode, Extension, Json};
use slug::slugify;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::{
    errors::GlobalAppError,
    middlewares::GlobalAppState,
    models::transactions::{GetCategoryId, TransactionRequest},
};

pub async fn add_expense(
    state: State<GlobalAppState>,
    Extension(uuid): Extension<Uuid>,
    Json(transactions): Json<Vec<TransactionRequest>>,
) -> Result<String, GlobalAppError> {
    let mut tx = state.pool.begin().await.map_err(|_| {
        GlobalAppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "database error!".to_string(),
        )
    })?;

    for transaction in transactions {
        let cat_slug = slugify(transaction.category);

        let cat_id = query_as::<_, GetCategoryId>(
            "SELECT id FROM categories WHERE slug = $1 AND user_id = $2",
        )
        .bind(cat_slug)
        .bind(uuid)
        .fetch_one(&mut *tx)
        .await
        .map_err(|error| match error {
            sqlx::Error::RowNotFound => {
                GlobalAppError::new(StatusCode::BAD_REQUEST, "category not found!".to_string())
            }
            _ => GlobalAppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "database error!".to_string(),
            ),
        })?;

        query("INSERT INTO expenses (user_id, category_id, description, amount, transaction_date) VALUES ($1, $2, $3, $4, $5)")
            .bind(uuid)
            .bind(cat_id.id)
            .bind(transaction.description)
            .bind(transaction.amount)
            .bind(transaction.transaction_date)
            .execute(&mut *tx)
            .await
            .map_err(|_| {
                GlobalAppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database error!".to_string(),
                )
            })?;
    }

    tx.commit().await.map_err(|_| {
        GlobalAppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "database error!".to_string(),
        )
    })?;

    Ok("Expenses updated successfully!".to_string())
}
