// /Users/matt/external_code/BrooksYew/brooks-full-stack/backend/nodejs/express/database/userQueries.js
// /Users/matt/Documents/Programming/rust/postgres-test/src/main.rs

//use tokio_postgres::{NoTls, Error, SimpleQueryMessage};
use crate::database::UserId;
use crate::routes::errors::TodoAppError;
use crate::routes::users::{UserCreatedInfo, User};

use chrono::NaiveDateTime;
use deadpool_postgres::Pool;

pub async fn db_create_user(
    username: &str,
    password: &str,
    token: &str,
    db_pool: &Pool,
) -> Result<UserCreatedInfo, TodoAppError> {
    let con = db_pool.get().await.unwrap();
    let sql = "INSERT INTO users (username, password, token) VALUES ($1, $2, $3)";
    let err = con
        .execute(
            sql,
            &[
                &username.to_string(),
                &password.to_string(),
                &token.to_string(),
            ],
        )
        .await;

    if err.is_ok() {
        let result = UserCreatedInfo {
            id: 99,
            username: username.to_string(),
            token: token.to_string(),
        };
        Ok(result)
    } else {
        Err(TodoAppError {
            name: "error from executing sql",
        })
    }
}

pub fn db_hash_password(password: &str) {
    todo!()
}

pub async fn db_get_by_username(
    username: &str,
    db_pool: &Pool
) -> Result<User, TodoAppError> {
    let con = db_pool.get().await.unwrap();
    let sql = "SELECT * FROM users WHERE username = $1 LIMIT 1";
    let result = con.query(sql, &[&username.to_string()]).await;
    if let Ok(r) = result {
        let mut result = User::default();
        if let Some(user_row) = r.first() {
            //let date: NaiveDateTime = user_row.get("deleted_at");
            //println!("date string: {}", date);
            result.id = user_row.get("id");
            result.username = user_row.get("username");
            result.password = user_row.get("password");
            result.deleted_at = Some(user_row.get("deleted_at"));
            result.token = user_row.get("token");
        }
        Ok(result)
    } else {
        Err(TodoAppError {
            name: "invalid username or password",
        })
    }
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
