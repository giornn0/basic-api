use std::{
    mem::drop,
    sync::atomic::{AtomicUsize, Ordering},
};

use futures::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::mpsc::unbounded_channel;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{
    ws::{Message, WebSocket, Ws},
    Filter, Rejection, Reply,
};

use crate::{
    application::users::service::get_user,
    config::{DBPool, LogModel, WsConnection},
    core::tokens::{AuthPayload, FromToken},
    utils::database::get_ws_pool,
};

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

pub fn get_ws_handler() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let connections = WsConnection::default();

    //passing into a filter
    let connections = warp::any().map(move || connections.clone());
    let token = warp::header("Sec-WebSocket-Protocol").map(move |token: String| token);

    warp::path("ws")
        .and(warp::ws())
        .and(connections)
        .and(token)
        .map(|ws: Ws, connections, token: String| {
            let ws = ws.max_message_size(1024);
            ws.on_upgrade(move |socket| connection_handler(connections, socket, token))
        })
}

async fn connection_handler(connections: WsConnection, ws: WebSocket, token: String) {
    let auth = &AuthPayload::from_token(token, Some(true)).unwrap();
    let pool = get_ws_pool().unwrap();
    let uuid = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);
    let user = get_user(uuid as i32, &pool).unwrap();

    drop(pool);

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

    connections.write().await.insert(uuid, (*auth, tx));
    eprintln!("User {} connected successfully", user.fullname());

    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("Error while sending message {}", e);
                break;
            }
        };
        eprintln!("{:?}", &msg);
        message_handler(uuid, user.fullname(), msg, &connections).await;
    }

    disconect_handler(uuid, user.fullname(), &connections).await;
}

async fn message_handler(
    current: usize,
    current_name: String,
    msg: Message,
    connections: &WsConnection,
) {
    let msg = if let Ok(string) = msg.to_str() {
        string
    } else {
        return;
    };

    let new_message = format!("<User-{}> : {}", current_name, msg);

    for (&uid, tx) in connections.read().await.iter() {
        if current != uid {
            if let Err(e) = tx.1.send(Message::text(new_message.clone())) {
                eprintln!("Error while trying to send message. {}", e);
            }
        }
    }
}

async fn disconect_handler(current: usize, current_name: String, connections: &WsConnection) {
    eprintln!("User {} has been disconnected", current_name);
    connections.write().await.remove_entry(&current);
}
