// /Users/matt/external_code/BrooksYew/brooks-full-stack/backend/nodejs/express/database/userQueries.js
// /Users/matt/Documents/Programming/rust/postgres-test/src/main.rs

//use tokio_postgres::{NoTls, Error, SimpleQueryMessage};
use crate::database::{get_db_con, DBPool, TodoDBError, UserId};
use crate::routes::users::UserCreatedInfo;

pub async fn db_create_user(
    username: &str,
    password: &str,
    token: &str,
    db_pool: &DBPool) -> Result<UserCreatedInfo, TodoDBError> {

    let con = get_db_con(db_pool).await?;
    let sql = "INSERT INTO users (username, password, token) VALUES ($1, $2, $3)";
    con.execute(sql, &[&username.to_string(), &password.to_string(), &token.to_string()])
        .await
        .map_err(TodoDBError::DBQueryError)?;

    let result = UserCreatedInfo {
        id: 99,
        username: username.to_string(),
        token: token.to_string(),
    };
    Ok(result)
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
