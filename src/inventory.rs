use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    routing::{get, put},
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

#[derive(Deserialize)]
struct EditInventoryItem {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    stock: Option<i32>,
}

/// Create a router for the inventory
pub(crate) fn router() -> Router {
    Router::new()
        .route("/", get(list).post(create))
        .route("/:id", put(edit).delete(remove))
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

/// Edit the properties of an inventory item
async fn edit(
    Path(id): Path<i32>,
    Json(payload): Json<EditInventoryItem>,
    Extension(pool): Extension<PgPool>,
) -> StatusCode {
    let mut conn = pool.acquire().await.unwrap();

    // Ensure the item exists
    let item = query_as!(InventoryItem, "select * from inventory where id = $1", id)
        .fetch_one(&mut conn)
        .await
        .unwrap();

    // Change the values in the database
    query!(
        "update inventory set name = $1, description = $2, stock = $3",
        payload.name.unwrap_or(item.name),
        payload.description.unwrap_or(item.description),
        payload.stock.unwrap_or(item.stock)
    )
    .execute(&mut conn)
    .await
    .unwrap();

    StatusCode::NO_CONTENT
}

/// Delete an inventory item by its ID
async fn remove(Path(id): Path<i32>, Extension(pool): Extension<PgPool>) -> StatusCode {
    let mut conn = pool.acquire().await.unwrap();

    query!("delete from inventory where id = $1", id)
        .execute(&mut conn)
        .await
        .unwrap();

    StatusCode::NO_CONTENT
}
