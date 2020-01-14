use chrono::{NaiveDateTime, Local, Duration, Utc};
use crate::schema::users;
use serde::{Deserialize,Serialize};
use crate::error::ServiceError;
use crate::models::{Validate, re_test_name, re_test_psw};
use actix_web::{Error, error, FromRequest, HttpRequest};
use actix::Message;
use jsonwebtoken::{encode, Header, decode, Validation};
use crate::models::msg::Msg;
use actix_web::dev::Payload;

pub const LIMIT_PERMIT: i16 = 0x01;  // follow,star...
pub const BASIC_PERMIT: i16 = 0x02;  // create, edit self created...
pub const EIDT_PERMIT: i16 = 0x04;   // edit/del others' creats
pub const MOD_PERMIT: i16 = 0x10;    // mod role
pub const ADMIN_PERMIT: i16 = 0x80;  // admin

#[derive(Debug,Clone, Serialize, Deserialize, PartialEq, Identifiable, Queryable, Insertable)]
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
    pub auth_from: String, // for OAuth
    pub email_confirmed: bool
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
    type Result = Result<Msg, ServiceError>;
}

//// auth via token
//impl FromRequest for CheckUser {
//    type Config = ();
//    type Error = ServiceError;
//    type Future = Result<CheckUser, ServiceError>;
//
//    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
//        if let Some(auth_token) = req.headers().get("authorization") {
//            if let Ok(auth) = auth_token.to_str() {
//                let user: CheckUser = decode_token(auth)?;
//                return Ok(user);
//            }
//        }
//        Err(ServiceError::Unauthorized.into())
//    }
//}

#[derive(Debug, Deserialize,Serialize)]
pub struct AuthUser {
    pub uname: String,
    pub password: String,
}

impl Validate for AuthUser{
    fn validate(&self) -> Result<(), Error> {
        let uname=&self.uname;
        let password=&self.password;
        let check=&uname.trim().len()< &16  && &password.trim().len()< &16 ;
        if check{
            Ok(())
        }else{
            Err(error::ErrorBadRequest("Invalid username or password"))
        }
    }
}

impl Message for AuthUser {
    type Result = Result<CheckUser,ServiceError>;
}


// jwt Token auth: Claim, token
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String, // issuer
    pub sub: String, // subject
    pub iat: i64,    // issued at
    pub exp: i64,    // expiry
    pub uid: String, // user id
    pub uname: String,
}

// claims's constructor
impl Claims {
    pub fn new(uid: &str, uname: &str) -> Self {
        Claims {
            iss: "ruthub".into(),
            sub: "auth".into(),
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(24 * 5)).timestamp(),
            uid: uid.to_owned(),
            uname: uname.to_owned(),
        }
    }
}

fn get_secret() -> String {
    dotenv::var("SECRET_KEY").unwrap_or_else(|_| "AHaRdGuESsSeCREkY".into())
}

pub fn encode_token(data: &CheckUser) -> Result<String, ServiceError> {
    let claims = Claims::new(data.id.as_str(), data.uname.as_str());
    encode(&Header::default(), &claims, get_secret().as_ref())
        .map_err(|_err| ServiceError::InternalServerError("encode".into()))
}

pub fn decode_token(token: &str) -> Result<CheckUser, ServiceError> {
    decode::<Claims>(token, get_secret().as_ref(), &Validation::default())
        .map(|data| Ok(data.claims.into()))
        .map_err(|_err| ServiceError::Unauthorized)?
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegUser {
    pub uname: String,
    pub password: String,
    pub confirm: String,//验证码
}

impl Message for RegUser {
    type Result = Result<Msg, ServiceError>;
}

impl Validate for RegUser {
    fn validate(&self) -> Result<(), Error> {
        let uname = &self.uname;
        let psw = &self.password;
        let check = re_test_name(uname) && re_test_psw(psw);

        if check {
            Ok(())
        } else {
            Err(error::ErrorBadRequest("Invalid username or password"))
        }
    }
}
