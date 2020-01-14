use crate::db::Dba;
use crate::error::ServiceError;
use crate::models::msg::Msg;
use crate::models::user::{AuthUser, CheckUser, RegUser, User};
use actix::Handler;
use diesel::prelude::*;
use crate::utils::hash_password;
use failure::Error;
use uuid::Uuid;


// /signin handle
impl Handler<AuthUser> for Dba {
    type Result = Result<CheckUser, Error>;

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


//signup handle
impl Handler<RegUser> for Dba {
    type Result = Result<Msg, Error>;

    fn handle(&mut self, msg: RegUser, _: &mut Self::Context) -> Self::Result {
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
                let psw = hash_password(&msg.password);
                // generate uuid as user.id
                let uid = Uuid::new_v4().to_string();
                let unm = msg.uname;
                let new_user = User::new(uid, unm, psw);
                diesel::insert_into(users).values(&new_user).execute(conn)?;
                Ok(Msg {
                    status: 201,
                    message: "success".to_string(),
                })
            }
        }
    }
}
