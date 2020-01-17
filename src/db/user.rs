use crate::db::Dba;
use crate::error::ServiceError;
use crate::models::msg::Msg;
use crate::models::user::{AuthUser, CheckUser, QueryUser, RegUser, User, UpdateUser, ChangePsw};
use crate::utils::{hash_password, verify_password};
use actix::{Handler, Actor};
use diesel::prelude::*;
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
            if verify_password(&msg.password, &check_user.password) {
                return Ok(check_user.into());
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

//query user handle
impl Handler<QueryUser> for Dba {
    type Result = Result<CheckUser, Error>;

    fn handle(&mut self, msg: QueryUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn = self.0.get()?;
        let query_user = users
            .filter(&uname.eq(&msg.uname))
            .get_result::<User>(&conn)?;
        Ok(query_user.into())
    }
}

//update user handle
impl Handler<UpdateUser> for Dba{
    type Result = Result<CheckUser, Error>;

    fn handle(&mut self, msg: UpdateUser, ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn=&self.0.get()?;
        let update_user=diesel::update(users.filter(&uname.eq(&msg.uname)))
            .set(&msg)
            .get_result::<User>(conn)?;
        Ok(update_user.into())
    }
}

//change password handle
impl Handler<ChangePsw> for Dba{
    type Result = Result<Msg,Error>;

    fn handle(&mut self, msg: ChangePsw, ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn=&self.0.get()?;
        let check_user=users
            .filter(&uname.eq(&msg.uname))
            .load::<User>(conn)?
            .pop();
        if let Some(old)=check_user{
            let new_password=hash_password(&msg.new_psw);
            diesel::update(&old)
                .set(password.eq(new_password))
                .execute(conn)?;
            Ok(Msg{
                status:200,
                message:"Success".to_string()
            })
        }else{
            Ok(Msg{
                status:404,
                message:"No Existing".to_string()
            })
        }
    }
}