use axum::{extract::State, http::StatusCode, Extension, Json};
use slug::slugify;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::{
    errors::GlobalAppError,
    middlewares::GlobalAppState,
    models::categories::{CreateCategoryDetails, GetUserCategories},
};

// todo : disallow adding same categories multiple times
pub async fn create_category(
    State(state): State<GlobalAppState>,
    Extension(uuid): Extension<Uuid>,
    Json(categories): Json<Vec<CreateCategoryDetails>>,
) -> Result<String, GlobalAppError> {
    let mut tx = state.pool.begin().await.map_err(|_| {
        GlobalAppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "database error!".to_string(),
        )
    })?;

    for category in categories {
        let slug = slugify(category.name.clone());

        query("INSERT INTO categories (user_id, name, slug, type, is_savings) VALUES ($1, $2, $3, $4, $5)")
            .bind(uuid)
            .bind(category.name)
            .bind(slug)
            .bind(category.category_type)
            .bind(category.is_savings)
            .execute(&mut *tx)
            .await
            .map_err(|error| {
                eprintln!("{:#?}", error);
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

    Ok("Categories inserted successfully!".to_string())
}

pub async fn list_categories(
    State(state): State<GlobalAppState>,
    Extension(uuid): Extension<Uuid>,
) -> Result<Json<Vec<GetUserCategories>>, GlobalAppError> {
    Ok(Json(
        query_as::<_, GetUserCategories>(
            "SELECT id, name, created_at, type, is_savings FROM categories WHERE user_id = $1",
        )
        .bind(uuid)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| {
            GlobalAppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "database error!".to_string(),
            )
        })?,
    ))
}
