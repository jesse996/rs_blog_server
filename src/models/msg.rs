use crate::models::user::CheckUser;
use serde::{Serialize,Deserialize};

// general response msg struct
#[derive(Deserialize, Serialize, Debug)]
pub struct Msg {
    pub status: i32,
    pub message: String,
}

// msg for login
#[derive(Deserialize, Serialize, Debug)]
pub struct AuthMsg {
    pub status: i32,
    pub message: String,
    pub token: String,
    pub exp: i32,
    pub user: CheckUser,
}

