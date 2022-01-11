use axum::http::StatusCode;
use sqlx::Error as SqlxError;
use tracing::error;

pub type Result<T> = std::result::Result<T, (StatusCode, &'static str)>;

/// Convert an SQLx error to a response
pub fn sqlx_error(err: SqlxError) -> (StatusCode, &'static str) {
    match err {
        SqlxError::RowNotFound => (StatusCode::NOT_FOUND, "not found"),
        e => {
            error!(error = %e, "an unexpected error occurred");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an unexpected error occurred",
            )
        }
    }
}
