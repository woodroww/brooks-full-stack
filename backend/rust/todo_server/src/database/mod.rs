pub mod user_queries;
pub mod task_queries;

pub type UserId = i32;
pub type TaskId = i32;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoDBError {
    #[error("error getting connection from DB pool: {0}")]
    DBPoolError(tokio_postgres::Error),
    #[error("error executing DB query: {0}")]
    DBQueryError(#[from] tokio_postgres::Error),
    #[error("error creating table: {0}")]
    DBInitError(tokio_postgres::Error),
    #[error("error reading file: {0}")]
    ReadFileError(#[from] std::io::Error),
}

impl actix_web::error::ResponseError for TodoDBError {}
