use crate::db::Dba;
use crate::error::ServiceError;
use crate::models::msg::Msg;
use crate::models::user::{AuthUser, CheckUser, RegUser, User};
use actix::Handler;
use argon2::Config;
use diesel::prelude::*;
//use std::error::Error;
use std::time::SystemTime;


//sign in handle
impl Handler<AuthUser> for Dba {
    type Result = Result<CheckUser, ServiceError>;

    fn handle(&mut self, msg: AuthUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn = &self.0.get()?;

        let query_user = users
            .filter(uname.eq(&msg.uname))
            .load::<User>(&*conn)?
            .pop();
        //      验证密码
        if let Some(check_user) = query_user {
            match argon2::verify_encoded(&check_user.password, &msg.password.as_bytes()) {
                Ok(valid) if valid => {
                    return Ok(check_user.into());
                }
                _ => ()
            }
        }
        Err(ServiceError::BadRequest("Auth fail".to_string()).into())
    }
}

fn hash_password(plain: &str) -> String {
    let password = plain.as_bytes();
    let salt = b"jesse233";
    let config = Config::default();
    argon2::hash_encoded(password, salt, &config).unwrap()
}


impl Handler<RegUser> for Dba {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, msg: RegUser, ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn = &self.0.get()?;
        let check_user = users
            .filter(&uname.eq(&msg.uname))
            .load::<User>(conn)?
            .pop();
        match check_user {
            Some(_) => Ok(Msg {
                status: 409,
                message: "Duplicated".to_string(),
            }),
            None => {
                //hash password
                let pswd = hash_password(&msg.password);
                // generae uuid as user.id
                let uid = uuid::Uuid::new_v4().to_string();
                let unm = msg.uname;
                let new_user = User::new(uid, unm, pswd);
                diesel::insert_into(users).values(&new_user).execute(conn)?;
                Ok(Msg {
                    status: 201,
                    message: "success".to_string(),
                })
            }
        }
    }
}
