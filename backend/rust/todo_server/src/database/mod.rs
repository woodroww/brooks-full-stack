pub mod user_queries;
pub mod task_queries;

use thiserror::Error;
use deadpool_postgres::Pool;
use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;
use crate::routes::users::UserInfo;
use actix_web::HttpRequest;

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

    pub async fn db_get_by_token(&self, token: &str) -> Option<UserInfo> {
        let con = self.pool.get().await.unwrap();
        let sql = "SELECT id, username, token FROM users WHERE token = $1 LIMIT 1";
        let result = con.query(sql, &[&token.to_string()]).await;
        if let Ok(r) = result {
            if let Some(user_row) = r.first() {
                return Some(UserInfo {
                    id: user_row.get("id"),
                    username: user_row.get("username"),
                    token: user_row.get("token"),
                });
            }
        }
        None
    }

    //pub async fn authenticate(&self, token: &str) -> Option<UserCreatedInfo> {
    //    self.db_get_by_token(token).await
    //}

    pub async fn authenticate(
        &self,
        req: &HttpRequest,
    ) -> Option<UserInfo> {
        let token = req.headers().get("x-auth-token");
        if let Some(t) = token {
            if let Some(token_string) = t.to_str().ok() {
                return self.db_get_by_token(token_string).await;
            }
        }
        None
    }
}


