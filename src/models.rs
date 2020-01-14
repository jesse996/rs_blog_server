use actix_web::Error;


pub mod user;
pub mod msg;


pub trait Validate {
    fn validate(&self) -> Result<(), Error>;
}
