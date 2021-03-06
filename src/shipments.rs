use crate::util::{sqlx_error, Result};
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Acquire, PgPool};

#[derive(Deserialize, Serialize)]
struct Shipment {
    #[serde(skip_deserializing)]
    id: i32,
    name: String,
    shipped: bool,
}

#[derive(Deserialize, Serialize)]
struct ShipmentItem {
    item_id: i32,
    count: i32,
}

/// Create a router for the shipments
pub(crate) fn router() -> Router {
    Router::new()
        .route("/", get(list).post(create))
        .route("/:id", get(read).delete(remove))
        .route("/:id/items", put(change_inventory))
        .route("/:id/items/:item_id", delete(remove_inventory))
        .route("/:id/ship", post(ship))
}

/// Get a list of all shipments in the database
/// Does not include the items in a shipment
async fn list(Extension(pool): Extension<PgPool>) -> Result<Json<Vec<Shipment>>> {
    let mut conn = pool.acquire().await.map_err(sqlx_error)?;

    let shipments = query_as!(Shipment, "select * from shipments")
        .fetch_all(&mut conn)
        .await
        .map_err(sqlx_error)?;
    Ok(Json(shipments))
}

/// Create a new shipment in the database
async fn create(
    Json(payload): Json<Shipment>,
    Extension(pool): Extension<PgPool>,
) -> Result<StatusCode> {
    let mut conn = pool.acquire().await.map_err(sqlx_error)?;

    query!("insert into shipments (name) values ($1)", payload.name)
        .execute(&mut conn)
        .await
        .map_err(sqlx_error)?;

    Ok(StatusCode::CREATED)
}

/// Get all the inventory in a shipment
async fn read(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<ShipmentItem>>> {
    let mut conn = pool.acquire().await.map_err(sqlx_error)?;

    let inventory = query_as!(
        ShipmentItem,
        "select item_id, count from shipment_inventory where shipment_id = $1",
        id
    )
    .fetch_all(&mut conn)
    .await
    .map_err(sqlx_error)?;

    Ok(Json(inventory))
}

/// Delete a shipment and all of its data
async fn remove(Path(id): Path<i32>, Extension(pool): Extension<PgPool>) -> Result<StatusCode> {
    let mut conn = pool.acquire().await.map_err(sqlx_error)?;

    query!("delete from shipments where id = $1", id)
        .execute(&mut conn)
        .await
        .map_err(sqlx_error)?;

    Ok(StatusCode::NO_CONTENT)
}

/// Change the amount of inventory in a shipment
/// If the amount is changed to 0, it is removed from the shipment
async fn change_inventory(
    Path(id): Path<i32>,
    Json(payload): Json<ShipmentItem>,
    Extension(pool): Extension<PgPool>,
) -> Result<StatusCode> {
    let mut conn = pool.acquire().await.map_err(sqlx_error)?;

    // Remove the inventory from the shipment if the amount is 0, otherwise perform an upsert
    if payload.count == 0 {
        query!(
            "delete from shipment_inventory where shipment_id = $1 and item_id = $2",
            id,
            payload.item_id
        )
        .execute(&mut conn)
        .await
        .map_err(sqlx_error)?;
    } else {
        query!(
            "insert into shipment_inventory values ($1, $2, $3) on conflict (shipment_id, item_id) do update set count = $3",
            id,
            payload.item_id,
            payload.count
        )
        .execute(&mut conn)
        .await
        .map_err(sqlx_error)?;
    }

    Ok(StatusCode::NO_CONTENT)
}

/// Remove an inventory item from the shipment
async fn remove_inventory(
    Path((shipment_id, item_id)): Path<(i32, i32)>,
    Extension(pool): Extension<PgPool>,
) -> Result<StatusCode> {
    let mut conn = pool.acquire().await.map_err(sqlx_error)?;

    query!(
        "delete from shipment_inventory where shipment_id = $1 and item_id = $2",
        shipment_id,
        item_id
    )
    .execute(&mut conn)
    .await
    .map_err(sqlx_error)?;

    Ok(StatusCode::NO_CONTENT)
}

/// "Ship" a shipment and remove all the inventory
/// Fails if the shipment has already been sent or there is not enough inventory
async fn ship(Path(id): Path<i32>, Extension(pool): Extension<PgPool>) -> Result<StatusCode> {
    let mut conn = pool.acquire().await.map_err(sqlx_error)?;

    // Find the shipment
    let shipment = query_as!(Shipment, "select * from shipments where id = $1", id)
        .fetch_one(&mut conn)
        .await
        .map_err(sqlx_error)?;
    if shipment.shipped {
        return Err((StatusCode::CONFLICT, "already shipped"));
    }

    // Get all the inventory in the shipment
    let inventory = query_as!(
        ShipmentItem,
        "select item_id, count from shipment_inventory where shipment_id = $1",
        shipment.id
    )
    .fetch_all(&mut conn)
    .await
    .map_err(sqlx_error)?;

    // Operate in transaction
    let mut txn = pool.begin().await.map_err(sqlx_error)?;

    // Update all the inventory items based on the shipment requirements
    for item in inventory {
        let conn = txn.acquire().await.map_err(sqlx_error)?;
        query!(
            "update inventory set stock = stock - $1 where id = $2",
            item.count,
            item.item_id
        )
        .execute(conn)
        .await
        .map_err(sqlx_error)?;
    }

    // Mark the shipment as complete
    let conn = txn.acquire().await.map_err(sqlx_error)?;
    query!("update shipments set shipped = true where id = $1", id)
        .execute(conn)
        .await
        .map_err(sqlx_error)?;

    // Commit the changes
    txn.commit().await.map_err(sqlx_error)?;

    Ok(StatusCode::NO_CONTENT)
}
