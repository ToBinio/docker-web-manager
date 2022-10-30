use std::sync::Mutex;
use actix_web::{get, post, Responder};
use actix_web::web::{Data, Json};
use serde::{Deserialize, Serialize};

pub struct Containers {
    pub containers: Mutex<Vec<Container>>,
}

#[derive(Serialize, Deserialize)]
pub struct Container {
    name: String,
}

#[get("/containers")]
pub async fn get_all_containers(containers: Data<Containers>) -> String {
    let containers = &*containers.containers.lock().unwrap();
    serde_json::to_string(containers).unwrap()
}

#[post("/container")]
pub async fn add_container(containers: Data<Containers>, container: Json<Container>) -> impl Responder {
    let containers = &mut *containers.containers.lock().unwrap();

    containers.push(container.0);

    println!("{}", containers.len());

    ""
}
