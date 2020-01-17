use chrono::{NaiveDateTime, Local, Duration, Utc};
use crate::schema::users;
use serde::{Deserialize, Serialize};
use crate::error::ServiceError;
use crate::models::Validate;
use actix_web::{FromRequest, HttpRequest};
use actix::Message;
use crate::models::msg::Msg;
use crate::utils::{re_test_name, re_test_psw, decode_token};
use failure::Error;
use std::pin::Pin;
use std::future::Future;
use actix_web::http::header;
use actix_web::dev::Payload;


pub const LIMIT_PERMIT: i16 = 0x01;
// follow,star...
pub const BASIC_PERMIT: i16 = 0x02;
// create, edit self created...
pub const EIDT_PERMIT: i16 = 0x04;
// edit/del others' creats
pub const MOD_PERMIT: i16 = 0x10;
// mod role
pub const ADMIN_PERMIT: i16 = 0x80;  // admin

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Identifiable, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub uname: String,
    pub password: String,
    pub join_at: NaiveDateTime,
    pub avatar: String,
    pub email: String,
    pub intro: String,
    pub location: String,
    pub nickname: String,
    pub permission: i16,
    pub link: String,
    // for OAuth
    pub auth_from: String,
    pub email_confirmed: bool,
}


impl User {
    // User's constructor
    pub fn new(id: String, uname: String, password: String) -> Self {
        User {
            id,
            uname,
            password,
            join_at: Utc::now().naive_utc(),
            avatar: "".to_owned(),
            email: "".to_owned(),
            intro: "".to_owned(),
            location: "".to_owned(),
            nickname: "".to_owned(),
            permission: LIMIT_PERMIT | BASIC_PERMIT,
            link: "".to_owned(),
            auth_from: "".to_owned(),
            email_confirmed: false,
        }
    }
    // check permission
    pub fn can(&self, permission: i16) -> bool {
        (self.permission & permission) == permission
    }
}


// return as user info w/o password
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Identifiable, Queryable)]
#[table_name = "users"]
pub struct CheckUser {
    pub id: String,
    pub uname: String,
    pub join_at: NaiveDateTime,
    pub avatar: String,
    pub email: String,
    pub intro: String,
    pub location: String,
    pub nickname: String,
}


impl From<Claims> for CheckUser {
    fn from(claims: Claims) -> Self {
        CheckUser {
            id: claims.uid,
            uname: claims.uname,
            join_at: Utc::now().naive_utc(), // ??
            avatar: "".to_owned(),
            email: "".to_owned(),
            intro: "".to_owned(),
            location: "".to_owned(),
            nickname: "".to_owned(),
        }
    }
}


impl From<User> for CheckUser {
    fn from(user: User) -> Self {
        CheckUser {
            id: user.id,
            uname: user.uname,
            join_at: user.join_at,
            avatar: user.avatar,
            email: user.email,
            intro: user.intro,
            location: user.location,
            nickname: user.nickname,
        }
    }
}


impl Message for CheckUser {
    type Result = Result<Msg, Error>;
}


lazy_static::lazy_static! {
    static ref BEARER_REGEXP : regex::Regex =  regex::Regex::new(r"^Bearer\s(.*)$").expect("Bearer regexp failed!");
}
// auth via token
impl FromRequest for CheckUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|v|v.to_str().ok())
            .and_then(|authorization| {
                BEARER_REGEXP
                    .captures(authorization)
                    .and_then(|captures| captures.get(1))
            })
            .map(|v| v.as_str());
        if let Some(auth) = token {
            let user = decode_token(auth);
            if let Ok(auth) = user {
                return Box::pin(async { Ok(auth) });
            }
        }
        Box::pin(async { Err(ServiceError::Unauthorized.into()) })
    }
}


#[derive(Debug, Deserialize)]
pub struct AuthUser {
    pub uname: String,
    pub password: String,
}


impl Validate for AuthUser {
    fn validate(&self) -> Result<(), ServiceError> {
        let uname = &self.uname;
        let password = &self.password;
        let check = &uname.trim().len() < &16 && &password.trim().len() < &16;
        if check {
            Ok(())
        } else {
            Err(ServiceError::BadRequest("Invalid username or password".into()))
        }
    }
}


impl Message for AuthUser {
    type Result = Result<CheckUser, Error>;
}


// jwt Token auth: Claim, token
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    // issuer
    pub sub: String,
    // subject
    pub iat: i64,
    // issued at
    pub exp: i64,
    // expiry
    pub uid: String,
    // user id
    pub uname: String,
}


// claims's constructor
impl Claims {
    pub fn new(uid: &str, uname: &str) -> Self {
        Claims {
            iss: "jesse".into(),
            sub: "auth".into(),
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(24 * 5)).timestamp(),
            uid: uid.to_owned(),
            uname: uname.to_owned(),
        }
    }
}


#[derive(Deserialize, Serialize, Debug)]
pub struct RegUser {
    pub uname: String,
    pub password: String,
    pub confirm: String,//验证码
}


impl Message for RegUser {
    type Result = Result<Msg, Error>;
}


impl Validate for RegUser {
    fn validate(&self) -> Result<(), ServiceError> {
        let uname = &self.uname;
        let psw = &self.password;
        let check = re_test_name(uname) && re_test_psw(psw);

        if check {
            Ok(())
        } else {
            Err(ServiceError::BadRequest("validate fail".into()))
        }
    }
}


pub struct QueryUser {
    pub uname: String
}


impl Message for QueryUser {
    type Result = Result<CheckUser, Error>;
}


#[derive(Deserialize,AsChangeset)]
#[table_name="users"]
pub struct UpdateUser {
    pub uname: String,
    // cannot change, just as id
    pub avatar: String,
    pub email: String,
    pub intro: String,
    pub location: String,
    pub nickname: String,
}

impl Validate for UpdateUser{
    fn validate(&self) -> Result<(), ServiceError> {
//        todo validate
        Ok(())
    }
}
impl Message for UpdateUser{
    type Result = Result<CheckUser, Error>;
}

#[derive(Deserialize)]
pub struct ChangePsw{
    pub uname:String,
    pub new_psw:String
}
impl Message for ChangePsw{
    type Result = Result<Msg,Error>;
}
impl Validate for ChangePsw{
    fn validate(&self) -> Result<(), ServiceError> {
//       todo
        Ok(())
    }
}