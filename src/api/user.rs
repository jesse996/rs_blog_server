use crate::db::DbAddr;
use crate::error::ServiceError;
use crate::models::msg::AuthMsg;
use crate::models::user::{AuthUser, RegUser};
use crate::models::Validate;
use crate::utils::encode_token;
use actix_web::web::{Data, Json};
use actix_web::{get, post, web, HttpResponse, Result};


#[post("/signin")]
pub async fn signin(auth: web::Json<AuthUser>, db: web::Data<DbAddr>) -> Result<HttpResponse> {
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
        Err(e) => {
            Err(e.downcast::<ServiceError>()?.into())
        }
    }
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


#[get("/posts")]
pub async fn posts() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("ok"))
}
