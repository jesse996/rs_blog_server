use actix::MailboxError;
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use diesel::r2d2::PoolError;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use failure::{Fail,Error};


#[derive(Debug, Fail)]
pub enum ServiceError {
    // 400
    #[fail(display =  "BadRequest: {}", _0)]
    BadRequest(String),

    // 401
        #[fail(display = "Unauthorized")]
    Unauthorized,

    // 404
    #[fail(display = "Not Found: {}",_0)]
    NotFound(String),

    // 500+
    #[fail(display = "Internal Server Error: {}",_0)]
    InternalServerError(String),
}

// impl ResponseError trait allows to convert errors into http responses with appropriate data
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

impl From<MailboxError> for ServiceError {
    fn from(_error: MailboxError) -> Self {
        ServiceError::InternalServerError("Mailbox".into())
    }
}

// uuid parse error
//impl From<ParseError> for ServiceError {
//    fn from(_: ParseError) -> ServiceError {
//        ServiceError::BadRequest("Invalid UUID".into())
//    }
//}

impl From<DieselError> for ServiceError {
    fn from(error: DieselError) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let msg = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::BadRequest(msg);
                }
                ServiceError::InternalServerError("datebase".into())
            }
            DieselError::NotFound => {
                ServiceError::NotFound("requested record was not found".into())
            }
            _ => ServiceError::InternalServerError("datebase".into()),
        }
    }
}

impl From<PoolError> for ServiceError {
    fn from(_error: PoolError) -> Self {
        ServiceError::InternalServerError("pool".into())
    }
}

//// Base64 decode
//impl From<DecodeError> for ServiceError {
//    fn from(_error: DecodeError) -> Self {
//        ServiceError::BadRequest("Invalid Base64 Code".into())
//    }
//}
