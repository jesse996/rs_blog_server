use crate::db::DbAddr;
use crate::error::ServiceError;
use crate::models::msg::AuthMsg;
use crate::models::user::{encode_token, AuthUser, RegUser};
use crate::models::Validate;
use actix::Response;
use actix_web::web::{Data, Json};
use actix_web::{error, get, post, web, HttpResponse, Responder, ResponseError, Result, Error};

#[post("/login")]
pub async fn login(auth: web::Json<AuthUser>, db: web::Data<DbAddr>) -> Result<HttpResponse> {
    let auth_user = auth.into_inner();
    auth_user.validate()?;
    let data = db.send(auth_user).await?;
    match data {
        Ok(user) => {
            let token = encode_token(&user)?;
            let auth_msg = AuthMsg {
                status: 200,
                message: "Success".to_string(),
                token,
                exp: 5, // unit: day
                user,
            };
            Ok(HttpResponse::Ok().json(auth_msg))
        }
        Err(e) => Err(e.into()),
    }
}


#[post("/signup")]
pub async fn signup(data: Json<RegUser>, db: Data<DbAddr>) -> Result<HttpResponse> {
    let data = data.into_inner();
    let res = db.send(data).await?;
    Ok(HttpResponse::Ok().body("123"))
//    match res {
//        Ok(data) => Ok(HttpResponse::Ok().json(data)),
//        Err(e) => Err(e.into())
//
//    }
}
