use warp::Filter;
use futures::StreamExt;
use futures::FutureExt;
use warp::Rejection;
use warp::Reply;

pub fn get_ws_handler()->impl Filter<Extract= impl Reply, Error=Rejection> + Clone{
    warp::path("ws").and(warp::ws()).map(|ws: warp::ws::Ws|{
        ws.on_upgrade(|websocket| {
            // // Just echo all messages back.
            let (tx, rx) = websocket.split();
            rx.forward(tx).map(|result| {
                println!("Exit");
                // if let Err(e) = result {
                //     eprintln!("websocket error: {:?}", e);
                // }
            })
        })
    })
}
