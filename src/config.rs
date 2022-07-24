use std::{sync::Arc, collections::HashMap};

use diesel::{r2d2::{PooledConnection, ConnectionManager}, PgConnection};
use diesel_derive_enum::DbEnum;
use serde::{Serialize, Deserialize};
use tokio::sync::{RwLock, mpsc::UnboundedSender};
use warp::ws::Message;

use crate::core::tokens::AuthPayload;

pub fn default_pager()->(i64,i64){
    (5,1)
}

pub type DBPool = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(DbEnum, Debug, Serialize, Deserialize, Clone, Hash,Eq, PartialEq)]
pub enum LogModel {
    User,   //'user
    Client, //'client
    Worker, //'worker
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash,Eq, PartialEq)]
pub enum Role {
    Admin,
    User,
    Client,
}

//Models that commnicates through ws
// #[derive(Serialize, Deserialize, Debug, Clone, Hash,Eq, PartialEq)]
// pub struct Message{
//     from: i32,
//     to: i32,
//     content: String
// }

pub type WsConnection = Arc<RwLock<HashMap<usize, UnboundedSender<Message>>>>;
pub type WsAuthPayload = Arc<RwLock<AuthPayload>>;
pub type WsDBPool = Arc<RwLock<DBPool>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct WsExtra{
    pub token: String,
}
