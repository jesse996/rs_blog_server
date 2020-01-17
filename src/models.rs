use crate::error::ServiceError;


pub mod user;
pub mod msg;


pub trait Validate {
    fn validate(&self) -> Result<(), ServiceError>;
}
