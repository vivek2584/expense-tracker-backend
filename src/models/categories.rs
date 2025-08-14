use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateCategoryDetails {
    pub name: String,
    pub category_type: CategoryType,
}

#[derive(Deserialize, sqlx::Type)]
#[sqlx(type_name = "category_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum CategoryType {
    Expense,
    Income,
}
