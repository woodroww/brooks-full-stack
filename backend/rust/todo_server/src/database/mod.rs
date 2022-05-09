pub mod user_queries;
pub mod task_queries;

use thiserror::Error;
use deadpool_postgres::Pool;
use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;

pub type UserId = i32;
pub type TaskId = i32;

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


pub struct TodoDB {
    pool: Pool,
}

impl TodoDB {

    pub fn new() -> Self {
        // postgresql://matt@localhost/brooks
        let mut config = Config::new();
        config.dbname = Some("brooks".to_string());
        config.user = Some("matt".to_string());
        config.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
        let pool = config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
        TodoDB { pool }
    }
}


