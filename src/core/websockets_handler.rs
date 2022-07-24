use std::sync::{
    atomic::{AtomicUsize, Ordering}, Arc, RwLock,
};

use futures::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::mpsc::unbounded_channel;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{
    ws::{WebSocket, Ws, Message},
    Filter, Rejection, Reply, reject::custom,
};

use crate::{config::{WsConnection, WsExtra}, core::tokens::{AuthPayload, FromToken}, utils::database::get_pool};

use super::server_model::Pool;

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

pub fn get_ws_handler(
    pool: &Arc<Pool>
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let connections = WsConnection::default();

    //passing into a filter
    let connections = warp::any().map(move || connections.clone());
    let auth =  warp::query().map(move|extras: WsExtra| {
        Arc::new(RwLock::new(AuthPayload::from_token(extras.token).map_err(custom)))
    });
    // let pool =  warp::any().map(move|| {
    //     Arc::new(RwLock::new(get_pool(pool.clone())))
    // });

    warp::path("ws")
        .and(warp::ws())
        .and(connections)
        .and(auth)
        // .and(pool)
        .map(|ws: Ws, connections, auth| {
            println!("{:?}",auth);
            ws.on_upgrade(move |socket| connection_handler(connections, socket))
        })
}

async fn connection_handler(connections: WsConnection, ws: WebSocket) {
    // if let Ok(success_pool) = get_pool(start_simple_db()) {
        // if let Ok(user) = get_user(
           let user = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed) ;
            // &success_pool,
        // ) {
            eprintln!("User {} connected successfully", user);

            let (mut user_ws_tx, mut user_ws_rx) = ws.split();

            let (tx, rx) = unbounded_channel();
            let mut rx = UnboundedReceiverStream::new(rx);

            tokio::task::spawn(async move {
                while let Some(message) = rx.next().await {
                    user_ws_tx
                        .send(message)
                        .unwrap_or_else(|e| {
                            eprintln!("Error {}", e);
                        })
                        .await
                }
            });

            // if let Ok(auth) = user.get_auth(LogModel::User){
            connections.write().await.insert(user, tx);
            
            while let Some(result)= user_ws_rx.next().await{
                let msg = match result {
                    Ok(msg) => msg,
                    Err(e)=>{
                        eprintln!("Error while sending message {}", e);
                        break;
                    }
                };
                message_handler(user, msg, &connections).await;
            }

            disconect_handler(user, &connections).await;

            // }
            // eprintln!("Error while getting the autorization")
        // }
    // };
    // eprintln!("Could not established a pool to the database");
}

async fn message_handler(current: usize, msg: Message, connections: &WsConnection){
    let msg = if let Ok(string) = msg.to_str(){string}else{return;};

    let new_message = format!("<User-{}> : {}", current, msg);

    for (&uid, tx) in connections.read().await.iter(){
        if current != uid{
            if let Err(e) = tx.send(Message::text(new_message.clone())){
                eprintln!("Error while trying to send message. {}", e);
            }
        }
    }
}

async fn disconect_handler(current: usize, connections: &WsConnection){
    eprintln!("User {} has been disconnected", current);
    connections.write().await.remove_entry(&current);
}