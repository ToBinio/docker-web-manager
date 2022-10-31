mod container;
mod websocket;

use std::sync::Mutex;
use actix::Actor;
use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{App, get, HttpServer, Result};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use crate::container::containers::{add_container, Containers, get_all_containers};
use crate::websocket::connection::{start_connection};
use crate::websocket::connections::WsConnections;

#[get("/")]
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open_async("./public/dist/public/index.html").await?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let ws_connections = Data::new(WsConnections {
        users: Default::default()
    }.start());

    let containers = Data::new(Containers {
        containers: Mutex::new(Vec::new()),
        connections: ws_connections.get_ref().clone(),
    }.start());

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(containers.clone())
            .app_data(ws_connections.clone())
            .service(index)
            .service(start_connection)
            .service(get_all_containers)
            .service(add_container)
            .service(Files::new("/", "./public/dist/public").show_files_listing())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}