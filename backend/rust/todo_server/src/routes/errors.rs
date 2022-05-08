use thiserror::Error;
use actix_web::error::ResponseError;

#[derive(Debug, Error)]
pub struct TodoAppError {
    pub name: &'static str
    //pub name: String,
    //source: actix_web::error::Error,
}

impl std::fmt::Display for TodoAppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TodoAppError {}", self.name)
    }
}

// actix_web Use default implementation for `error_response()` method
impl ResponseError for TodoAppError {}

