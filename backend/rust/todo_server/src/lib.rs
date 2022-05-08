pub mod routes;
pub mod database;

type UserId = i32;
type TaskId = i32;


use mobc::{async_trait, Manager};

#[derive(Debug)]
pub struct TodoDBError;

pub struct TodoDBConnection;

impl TodoDBConnection {
    pub async fn query(&self) -> String {
        "PONG".to_string()
    }
}

pub struct TodoDBConnectionManager;

#[async_trait]
impl Manager for TodoDBConnectionManager {
    type Connection = TodoDBConnection;
    type Error = TodoDBError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Ok(TodoDBConnection)
    }

    async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        Ok(conn)
    }
}
