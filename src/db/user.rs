use crate::db::Dba;
use crate::error::ServiceError;
use crate::models::msg::Msg;
use crate::models::user::{AuthUser, CheckUser, RegUser, User};
use actix::Handler;
use argon2::Config;
use diesel::prelude::*;
//use std::error::Error;
use std::time::SystemTime;
use failure::Error;

impl Handler<AuthUser> for Dba {
    type Result = Result<CheckUser, ServiceError>;

    fn handle(&mut self, msg: AuthUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn = &self.0.get()?;

        let query_user = users
            .filter(uname.eq(&msg.uname))
            .load::<User>(&*conn)?
            .pop();
        println!("auth user query result:  {:?}", query_user);
        //        todo:  验证密码
        Err(ServiceError::BadRequest("Auth fail".to_string()).into())
    }
}

fn hash_password(plain: &str) -> String {
    let password = plain.as_bytes();
    let salt = b"jesse233";
    let config = Config::default();
    let hash = argon2::hash_encoded(password, salt, &config).unwrap();
    let matches = argon2::verify_encoded(&hash, password).unwrap();
    assert!(matches);
    hash
}
#[test]
fn test() {
    let t1 = SystemTime::now();
    let s = hash_password("jesse");
    println!("cost time: {:?}", t1.elapsed().unwrap());
    println!("{}", s);
}

impl Handler<RegUser> for Dba {
    type Result = Result<Msg, Error>;

    fn handle(&mut self, msg: RegUser, ctx: &mut Self::Context) -> Self::Result {
        Ok(Msg{ status: 0, message: "aaa".to_string() })
//        use crate::schema::users::dsl::*;
//        let conn = &self.0.get()?;
//        let check_user = users
//            .filter(&uname.eq(&msg.uname))
//            .load::<User>(conn)?
//            .pop();
//        match check_user {
//            Some(_) => Ok(Msg {
//                status: 409,
//                message: "Duplicated".to_string(),
//            }),
//            None => {
//                //hash password
//                let pswd = blake3::hash(&msg.password.as_bytes()).to_hex().to_string();
//                // generae uuid as user.id
//                let uid = uuid::Uuid::new_v4().to_string();
//                let unm = msg.uname;
//                let new_user = User::new(uid, unm, pswd);
//                diesel::insert_into(users).values(&new_user).execute(conn)?;
//                Ok(Msg {
//                    status: 201,
//                    message: "success".to_string(),
//                })
//            }
//        }
    }
}
