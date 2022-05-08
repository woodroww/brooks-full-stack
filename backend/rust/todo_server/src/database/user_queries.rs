// /Users/matt/external_code/BrooksYew/brooks-full-stack/backend/nodejs/express/database/userQueries.js
// /Users/matt/Documents/Programming/rust/postgres-test/src/main.rs

use tokio_postgres::{NoTls, Error, SimpleQueryMessage};
use crate::UserId;


fn create_user(username: &str, password: &str, token: &str) {
    todo!()
}

fn hash_password(password: &str) {
    todo!()
}

fn get_by_username(username: &str) {
    todo!()
}

fn find_and_remove_token(token: &str) {
    todo!()
}

fn get_by_token(token: &str) {
    todo!()
}

fn add_token_to_user(token: &str, user_id: UserId) {
    todo!()
}
