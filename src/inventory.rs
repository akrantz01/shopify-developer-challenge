use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgPool};

#[derive(Deserialize, Serialize)]
struct InventoryItem {
    #[serde(skip_deserializing)]
    id: i32,
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    stock: i32,
}

/// Create a router for the inventory
pub(crate) fn router() -> Router {
    Router::new().route("/", get(list).post(create))
}

/// Get a list of all inventory items in the database
async fn list(Extension(pool): Extension<PgPool>) -> Json<Vec<InventoryItem>> {
    let mut conn = pool.acquire().await.unwrap();

    let inventory = query_as!(InventoryItem, "select * from inventory")
        .fetch_all(&mut conn)
        .await
        .unwrap();
    Json(inventory)
}

/// Create a new inventory item in the database
async fn create(
    Json(payload): Json<InventoryItem>,
    Extension(pool): Extension<PgPool>,
) -> StatusCode {
    let mut conn = pool.acquire().await.unwrap();

    query!(
        "insert into inventory (name, description, stock) values ($1, $2, $3)",
        payload.name,
        payload.description,
        payload.stock
    )
    .execute(&mut conn)
    .await
    .unwrap();

    StatusCode::CREATED
}
