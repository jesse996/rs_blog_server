use actix_web::Error;
use lazy_static::*;
use regex::Regex;

pub mod user;
pub mod msg;


pub trait Validate {
    fn validate(&self) -> Result<(), Error>;
}

// re test
// for re test uname
pub fn re_test_name(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^[\w-]{3,5}$").unwrap(); // let fail in test
    }
    RE.is_match(text)
}

// for re test psw
pub fn re_test_psw(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^[\w#@~%^$&*-]{8,18}$").unwrap(); // let fail in test
    }
    RE.is_match(text)
}