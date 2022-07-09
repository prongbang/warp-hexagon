use crate::core::error_postgres;

pub mod postgres;

pub type Result<T> = std::result::Result<T, error_postgres::Error>;
