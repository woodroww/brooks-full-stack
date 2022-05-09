// /Users/matt/external_code/BrooksYew/brooks-full-stack/backend/nodejs/express/database/userQueries.js
// /Users/matt/Documents/Programming/rust/postgres-test/src/main.rs

use crate::database::{UserId, TodoDB};
use crate::routes::errors::TodoAppError;
use crate::routes::users::{UserCreatedInfo, User};

impl TodoDB {

    // hash password and store username, token, and hashed password in db
    // get back unique user id from db and return UserCreatedInfo
    pub async fn db_create_user(
        &self,
        username: &str,
        password: &str,
        token: &str,
    ) -> Result<UserCreatedInfo, TodoAppError> {
        let con = self.pool.get().await.unwrap();
        let sql = "INSERT INTO users (username, password, token) VALUES ($1, $2, $3)";
        let err = con
            .execute(
                sql,
                &[
                    &username.to_string(),
                    &self.db_hash_password(password),
                    &token.to_string(),
                ],
            )
            .await;

        if err.is_err() {
            let e = err.err().unwrap();
            if let Some(db_err) = e.as_db_error() {
                return Err(TodoAppError {
                    name: db_err.message().to_string(),
                })
            } else {
                return Err(TodoAppError {
                    name: "error from db_create_user".to_string(),
                })
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
                })
            } else {
                return Err(TodoAppError {
                    name: "error from db_create_user trying to get id".to_string(),
                })
            }
        }

        let query_result = err.unwrap();
        if let Some(row) = query_result.first() {
            let result = UserCreatedInfo {
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

    pub fn db_hash_password(
        &self,
        password: &str) -> String {
        // TODO: hash
        password.to_string()
    }

    pub async fn db_get_by_username(
        &self,
        username: &str,
    ) -> Option<User> {
        let con = self.pool.get().await.unwrap();
        let sql = "SELECT * FROM users WHERE username = $1 LIMIT 1";
        let result = con.query(sql, &[&username.to_string()]).await;
        if let Ok(r) = result {
            let mut result = User::default();
            if let Some(user_row) = r.first() {
                result.id = user_row.get("id");
                result.username = user_row.get("username");
                result.password = user_row.get("password");
                result.deleted_at = user_row.get("deleted_at");
                result.token = user_row.get("token");
                return Some(result);
            }
        }
        None
    }

    pub fn db_find_and_remove_token(
        &self,
        token: &str) {
        todo!()
    }

    pub async fn db_get_by_token(
        &self,
        token: &str) {
        let con = self.pool.get().await.unwrap();
        let sql = "SELECT * FROM users WHERE token = $1 LIMIT 1";
        let result = con.query(sql, &[&token.to_string()]).await;

        if let Ok(r) = result {
        }
    }

    pub fn db_add_token_to_user(
        &self,
        token: &str,
        user_id: UserId) {
        todo!()
    }
}
