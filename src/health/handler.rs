use warp::{http::StatusCode, reject, Reply};

use crate::core::error_postgres::Error::DBQueryError;
use crate::core::result;
use crate::database::postgres;

pub async fn health_handler(db_pool: postgres::DBPool) -> result::Result<impl Reply> {
    let db = postgres::get_db_con(&db_pool)
        .await
        .map_err(|e| reject::custom(e))?;
    db.execute("SELECT 1", &[])
        .await
        .map_err(|e| reject::custom(DBQueryError(e)))?;
    Ok(StatusCode::OK)
}