use lazy_static::lazy_static;
use regex::Regex;
use crate::models::user::{CheckUser, Claims};
use crate::error::ServiceError;
use jsonwebtoken::{encode, Header, decode, Validation};
use argon2::Config;


// re test uname
pub fn re_test_name(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^[\w-]{3,5}$").unwrap();
    }
    RE.is_match(text)
}


// re test psw
pub fn re_test_psw(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^[\w#@~%^$&*-]{6,18}$").unwrap();
    }
    RE.is_match(text)
}


fn get_secret() -> String {
    dotenv::var("SECRET_KEY").unwrap_or_else(|_| "AHaRdGuESsSeCREkY".into())
}


pub fn encode_token(data: &CheckUser) -> Result<String, ServiceError> {
    let claims = Claims::new(data.id.as_str(), data.uname.as_str());
    encode(&Header::default(), &claims, get_secret().as_ref())
        .map_err(|_err| ServiceError::InternalServerError("encode error".into()))
}


pub fn decode_token(token: &str) -> Result<CheckUser, ServiceError> {
    decode::<Claims>(token, get_secret().as_ref(), &Validation::default())
        .map(|data| Ok(data.claims.into()))
        .map_err(|_err| ServiceError::Unauthorized)?
}


pub fn hash_password(plain: &str) -> String {
    let password = plain.as_bytes();
    let salt = b"jesse233";
    let config = Config::default();
    argon2::hash_encoded(password, salt, &config).expect("hash password fail")
}


pub fn verify_password(plain: &str, hashed: &str) -> bool {
    match argon2::verify_encoded(hashed, plain.as_bytes()) {
        Ok(valid) if  valid => {
            true
        }
        _ => {
            false
        }
    }
}