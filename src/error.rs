use actix_web::{HttpResponse, ResponseError};
use failure::{Fail};

#[derive(Debug, Fail)]
pub enum ServiceError {
    // 400
    #[fail(display = "Bad Request:{}", _0)]
    BadRequest(String),

    // 401
    #[fail(display = "Unauthorized")]
    Unauthorized,

    // 404
    #[fail(display = "Not Found: {}", _0)]
    NotFound(String),

    // 500+
    #[fail(display = "Internal Server Error: {}", _0)]
    InternalServerError(String),
}



impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            ServiceError::NotFound(ref message) => HttpResponse::NotFound().json(message),
        }
    }
}
