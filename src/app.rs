use std::sync::Arc;
use warp::Filter;

use crate::core::errors::handle_rejections;
use crate::core::server_model::Pool;
use crate::core::server_service::up_server;
use crate::core::websockets_handler::get_ws_handler;
use crate::utils::server::port;
use crate::work::users::router::users_router;

pub async fn app(pool: &Arc<Pool>) {
    let port = port().unwrap();

    let start = warp::get()
        .and(warp::path("api"))
        .and(warp::path::end())
        .and_then(up_server);


    let ws = get_ws_handler();
    let apis = start.or(users_router(&pool)).recover(handle_rejections);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type", "authorization"])
        .allow_methods(vec!["GET", "POST", "DELETE", "PUT", "OPTION"]);

    let routes = apis.with(cors).or(ws);

    let (addr, server) = warp::serve(routes).bind_with_graceful_shutdown(([0, 0, 0, 0], port), async {
        tokio::signal::ctrl_c()
            .await
            .expect("Not used Ctrl + C for close");
        println!("Shutting down Server");
    });
    println!("Server listening => {}", addr);

    server.await;
}
