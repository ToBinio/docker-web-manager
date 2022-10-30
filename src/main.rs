mod container;

use std::sync::Mutex;
use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{App, get, HttpServer, Result};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use env_logger::Env;
use crate::container::{add_container, Containers, get_all_containers};

#[get("/")]
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open_async("./public/dist/public/index.html").await?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let containers = Data::new(Containers {
        containers: Mutex::new(Vec::new())
    });

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(containers.clone())
            .service(index)
            .service(get_all_containers)
            .service(add_container)
            .service(Files::new("/", "./public/dist/public").show_files_listing())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}