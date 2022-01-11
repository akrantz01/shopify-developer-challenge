use axum::{extract::Extension, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};

#[derive(Deserialize, Serialize)]
struct InventoryItem {
    #[serde(skip_deserializing)]
    id: i32,
    name: String,
    description: String,
    stock: i32,
}

/// Create a router for the inventory
pub(crate) fn router() -> Router {
    Router::new().route("/", get(list))
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
