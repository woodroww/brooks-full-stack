pub mod user_queries;
pub mod task_queries;

use thiserror::Error;
use tokio_postgres::{NoTls, Config, Error};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::str::FromStr;
use std::result::Result;

pub type UserId = i32;
pub type TaskId = i32;

pub type DBPool = mobc::Pool<PgConnectionManager<NoTls>>;
pub type DBCon = mobc::Connection<PgConnectionManager<NoTls>>;


#[derive(Error, Debug)]
pub enum TodoDBError {
    #[error("error getting connection from DB pool: {0}")]
    DBPoolError(mobc::Error<tokio_postgres::Error>),
    #[error("error executing DB query: {0}")]
    DBQueryError(#[from] tokio_postgres::Error),
    #[error("error creating table: {0}")]
    DBInitError(tokio_postgres::Error),
    #[error("error reading file: {0}")]
    ReadFileError(#[from] std::io::Error),
}

impl actix_web::error::ResponseError for TodoDBError {}

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;

/*    let con = get_db_con(db_pool).await?;
    con.batch_execute(init_file.as_str())
        .await
        .map_err(DBInitError)?;
*/
pub async fn get_db_con(db_pool: &DBPool) -> Result<DBCon, TodoDBError> {
    db_pool.get().await.map_err(TodoDBError::DBPoolError)
}

pub fn create_pool(postgres_string: &str) -> Result<DBPool, mobc::Error<Error>> {
    let config = Config::from_str(postgres_string)?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(DBPool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .build(manager))
}
