use crate::db::DbAddr;
use crate::error::ServiceError;
use crate::models::msg::{AuthMsg, UserMsg};
use crate::models::user::{AuthUser, RegUser, QueryUser, CheckUser, UpdateUser, ChangePsw};
use crate::models::Validate;
use crate::utils::encode_token;
use actix_web::web::{Data, Json};
use actix_web::{get, post, put, delete, web, HttpResponse, Result};


#[post("/signin")]
pub async fn signin(auth: web::Json<AuthUser>, db: web::Data<DbAddr>) -> Result<HttpResponse> {
    let auth_user = auth.into_inner();
    auth_user.validate()?;
    let user = db.send(auth_user).await??;

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


#[post("/signup")]
pub async fn signup(data: Json<RegUser>, db: Data<DbAddr>) -> Result<HttpResponse> {
    let data = data.into_inner();
    let res = db.send(data).await?;
    match res {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(e) => Err(e.into()),
    }
}


#[get("/users/{uname}")]
pub async fn get(uname: web::Path<String>, db: web::Data<DbAddr>) -> Result<HttpResponse> {
    let uname = uname.into_inner();
    let user = db.send(QueryUser { uname }).await??;
    let user_msg = UserMsg {
        status: 200,
        message: "Success".to_string(),
        user,
    };
    Ok(HttpResponse::Ok().json(user_msg))
}


#[post("/users/{uname}")]
pub async fn update(auth: CheckUser, user: web::Json<UpdateUser>, db: web::Data<DbAddr>) -> Result<HttpResponse> {
    let up_user = user.into_inner();
    if auth.uname != up_user.uname {
        return Err(ServiceError::BadRequest("Permission deny!".to_string()).into());
    }
    up_user.validate()?;
    let user = db.send(up_user).await??;
    let token = encode_token(&user)?;
    let auth_msg = AuthMsg {
        status: 200,
        message: "Success".to_string(),
        token,
        exp: 0,
        user,
    };
    Ok(HttpResponse::Ok().json(auth_msg))
}


#[put("/users/{uname}")]
pub async fn change_pw(auth: CheckUser, psw: web::Json<ChangePsw>, db: web::Data<DbAddr>) -> Result<HttpResponse> {
    let user_psw = psw.into_inner();
    if auth.uname != user_psw.uname {
        return Err(ServiceError::BadRequest("Permission deny!".to_string()).into());
    }
    user_psw.validate()?;
    let res = db.send(user_psw).await??;
    Ok(HttpResponse::Ok().json(res))
}