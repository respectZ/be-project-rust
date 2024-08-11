#[macro_use]
mod logger;
mod db;
mod models;
mod response;
mod routes;
mod schema;

use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use listenfd;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db = match db::establish_connection() {
        Ok(db) => db,
        Err(e) => {
            error!("Failed to establish connection to database: {}", e);
            return Ok(());
        }
    };

    let mut listenfd = listenfd::ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone()))
            .configure(routes::init)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
        server.bind(host)?
    };

    info!("Starting server at http://{}", server.addrs()[0]);
    server.run().await
}
