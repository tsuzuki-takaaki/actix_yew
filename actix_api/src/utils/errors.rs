use actix_web::{ ResponseError };
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
  #[error("Failed to get connection")]
  ConnectionPoolError(#[from] r2d2::Error),

  #[error("Failed to SQL execution")]
  SQLiteError(#[from] rusqlite::Error),
}

impl ResponseError for MyError {}
