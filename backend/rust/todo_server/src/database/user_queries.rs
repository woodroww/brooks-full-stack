// TODO
/*
addTokenToUser(token, userId) {
*/

// /Users/matt/external_code/BrooksYew/brooks-full-stack/backend/nodejs/express/database/userQueries.js
// /Users/matt/Documents/Programming/rust/postgres-test/src/main.rs

use crate::database::{TodoDB, UserId};
use crate::routes::users::{User, UserInfo};
use crate::routes::TodoAppError;

impl TodoDB {
    // hash password and store username, token, and hashed password in db
    // get back unique user id from db and return UserCreatedInfo
    pub async fn create_user(
        &self,
        username: &str,
        password: &str,
        token: &str,
    ) -> Result<UserInfo, TodoAppError> {
        let con = self.pool.get().await.unwrap();
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

        if err.is_err() {
            let e = err.err().unwrap();
            if let Some(db_err) = e.as_db_error() {
                return Err(TodoAppError {
                    name: db_err.message().to_string(),
                });
            } else {
                return Err(TodoAppError {
                    name: "error from db_create_user".to_string(),
                });
            }
        }

        // database needs to return the id key after insertion
        let sql = "SELECT id FROM users WHERE username = $1";
        let err = con.query(sql, &[&username.to_string()]).await;

        if err.is_err() {
            let e = err.err().unwrap();
            if let Some(db_err) = e.as_db_error() {
                return Err(TodoAppError {
                    name: db_err.message().to_string(),
                });
            } else {
                return Err(TodoAppError {
                    name: "error from db_create_user trying to get id".to_string(),
                });
            }
        }

        let query_result = err.unwrap();
        if let Some(row) = query_result.first() {
            let result = UserInfo {
                id: row.get("id"),
                username: username.to_string(),
                token: token.to_string(),
            };
            Ok(result)
        } else {
            Err(TodoAppError {
                name: "error from db_create_user trying to get id no first row".to_string(),
            })
        }
    }

    pub async fn get_by_username(&self, username: &str) -> Option<User> {
        let con = self.pool.get().await.unwrap();
        let sql = "SELECT * FROM users WHERE username = $1 LIMIT 1";
        let result = con.query(sql, &[&username.to_string()]).await;
        if let Ok(r) = result {
            if let Some(user_row) = r.first() {
                return Some(User {
                    id: user_row.get("id"),
                    username: user_row.get("username"), 
                    password: user_row.get("password"),
                    deleted_at: user_row.get("deleted_at"), 
                    token: user_row.get("token"),
                });
            }
        }
        None
    }

    pub async fn find_and_remove_token(&self, token: &str) -> Result<u64, TodoAppError> {
        let con = self.pool.get().await.unwrap();
        let sql = "UPDATE users SET token = NULL WHERE token = $1";
        let result = con.execute(sql, &[&token.to_string()]).await;
        match result {
            Ok(row_count) => Ok(row_count),
            Err(_) => Err(TodoAppError {
                name: "problems setting token to null".to_string(),
            }),
        }
    }

    pub async fn add_token_to_user(
        &self,
        token: &str,
        user_id: UserId,
    ) -> Result<(), TodoAppError> {
        let con = self.pool.get().await.unwrap();
        let sql = "UPDATE users SET token = $1 WHERE id = $2";
        let result = con.execute(sql, &[&token.to_string(), &user_id]).await;
        if result.is_err() {
            return Err(TodoAppError {
                name: "problems setting token".to_string(),
            });
        }
        Ok(())
    }
}
