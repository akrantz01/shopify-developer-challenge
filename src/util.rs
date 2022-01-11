use axum::http::StatusCode;
use sqlx::postgres::PgDatabaseError;
use sqlx::Error as SqlxError;
use tracing::error;

pub type Result<T> = std::result::Result<T, (StatusCode, &'static str)>;

/// Convert an SQLx error to a response
pub fn sqlx_error(err: SqlxError) -> (StatusCode, &'static str) {
    match err {
        SqlxError::RowNotFound => (StatusCode::NOT_FOUND, "not found"),
        SqlxError::Database(e) => {
            let pg = e.downcast_ref::<PgDatabaseError>();

            // Handle check constraint failures separately
            if pg.code() == "23514" {
                if pg.constraint().unwrap() == "non_negative_stock"
                    && pg.table().unwrap() == "inventory"
                {
                    return (StatusCode::BAD_REQUEST, "not enough inventory to ship");
                }
            }

            error!(error = %e, "an unexpected error occurred");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an unexpected error occurred",
            )
        }
        e => {
            error!(error = %e, "an unexpected error occurred");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an unexpected error occurred",
            )
        }
    }
}
