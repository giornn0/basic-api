use diesel_derive_enum::DbEnum;
use serde::{Serialize, Deserialize};

pub fn default_pager()->(i64,i64){
    (5,1)
}

#[derive(DbEnum, Debug, Serialize, Deserialize, Clone)]
pub enum LogModel {
    User,   //'user
    Client, //'client
    Worker, //'worker
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
    Admin,
    User,
    Client,
}