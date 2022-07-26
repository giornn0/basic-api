use std::{sync::Arc, collections::HashMap};

use diesel::{r2d2::{PooledConnection, ConnectionManager}, PgConnection};
use diesel_derive_enum::DbEnum;
use serde::{Serialize, Deserialize};
use tokio::sync::{RwLock, mpsc::UnboundedSender};
use warp::ws::Message;

use crate::core::{tokens::AuthPayload};

pub fn default_pager()->(i64,i64){
    (5,1)
}

pub type DBPool = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(DbEnum, Debug, Serialize, Deserialize, Clone, Hash,Eq, PartialEq,Copy)]
pub enum LogModel {
    User,   //'user
    Client, //'client
    Worker, //'worker
}
impl Default for LogModel{
    fn default()->Self{
        LogModel::Worker
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Hash,Eq, PartialEq, Copy)]
pub enum Role {
    Admin,
    User,
    Client,
}
impl Default for Role{
    fn default()->Self{
        Role::Client
    }
}
//Models that commnicates through ws
// #[derive(Serialize, Deserialize, Debug, Clone, Hash,Eq, PartialEq)]
// pub struct Message{
//     from: i32,
//     to: i32,
//     content: String
// }

pub type WsConnection = Arc<RwLock<HashMap<usize, (AuthPayload,UnboundedSender<Message>)>>>;
