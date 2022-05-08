// /Users/matt/external_code/BrooksYew/brooks-full-stack/backend/nodejs/express/database/userQueries.js
// /Users/matt/Documents/Programming/rust/postgres-test/src/main.rs

//use tokio_postgres::{NoTls, Error, SimpleQueryMessage};
use crate::database::UserId;
use crate::routes::users::UserCreatedInfo;
use crate::routes::errors::TodoAppError;

use deadpool_postgres::Pool;

pub async fn db_create_user(
    username: &str,
    password: &str,
    token: &str,
    db_pool: &Pool) -> Result<UserCreatedInfo, TodoAppError> {

    let con = db_pool.get().await.unwrap();
    let sql = "INSERT INTO users (username, password, token) VALUES ($1, $2, $3)";
    let err = con.execute(sql, &[&username.to_string(), &password.to_string(), &token.to_string()]).await;

    if err.is_ok() {
        let result = UserCreatedInfo {
            id: 99,
            username: username.to_string(),
            token: token.to_string(),
        };
        Ok(result)
    } else {
        Err(TodoAppError { name: "error from executing sql" })
    }
}

pub fn db_hash_password(password: &str) {
    todo!()
}

pub fn db_get_by_username(username: &str) {
    todo!()
}

pub fn db_find_and_remove_token(token: &str) {
    todo!()
}

pub fn db_get_by_token(token: &str) {
    todo!()
}

pub fn db_add_token_to_user(token: &str, user_id: UserId) {
    todo!()
}
