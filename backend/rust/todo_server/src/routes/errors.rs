use derive_more::{Display, Error};
use actix_web::error::ResponseError;


#[derive(Debug, Display, Error)]
#[display(fmt = "users error: {}", name)]
pub struct TodoAppError {
    pub name: &'static str
}

// Use default implementation for `error_response()` method
impl ResponseError for TodoAppError {}

