use std::sync::Arc;
use warp::Filter;

use crate::application::login::login;
use crate::application::organizations::router::organizations_router;
use crate::application::users::router::users_router;
use crate::core::errors::handle_rejections;
use crate::core::server_model::Pool;
use crate::core::server_service::up_server;
use crate::core::websockets_handler::get_ws_handler;
use crate::utils::server::port;

pub async fn app(pool: &Arc<Pool>) {
    let port = port().unwrap();

    let start = warp::get()
        .and(warp::path("api"))
        .and(warp::path::end())
        .and_then(up_server)
        .or(login(pool));

    let ws = get_ws_handler();
    let apis = start
        .or(users_router(pool))
        .or(organizations_router(pool))
        .or(ws)
        .recover(handle_rejections);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type", "authorization"])
        .allow_methods(vec!["GET", "POST", "DELETE", "PUT", "OPTION"]);

    let routes = apis.with(cors);

    let (addr, server) = warp::serve(routes)
        //      .tls()
        //      .cert_path(".tls/cert.pem")
        //      .key_path(".tls/key.pem")
        .bind_with_graceful_shutdown(([0, 0, 0, 0], port), async {
            tokio::signal::ctrl_c()
                .await
                .expect("Not used Ctrl + C for close");
            println!("Shutting down Server");
        });
    println!("Server listening => {}", addr);

    server.await;
}
