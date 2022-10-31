use std::sync::Mutex;
use actix::Addr;
use actix_web::{get, HttpResponse, post};
use actix_web::web::{Data, Json};
use serde::{Deserialize, Serialize};
use crate::websocket::messages::{AddNewContainer, NewContainer, WsMessage};
use crate::websocket::websocket::WebsocketConnections;

pub struct Containers {
    pub containers: Mutex<Vec<Container>>,
}

#[derive(Serialize)]
pub struct Container {
    name: String,
}

#[get("/containers")]
pub async fn get_all_containers(containers: Data<Containers>) -> String {
    let containers = &*containers.containers.lock().unwrap();
    serde_json::to_string(containers).unwrap()
}

#[post("/container")]
pub async fn add_container(containers: Data<Containers>, connections: Data<Addr<WebsocketConnections>>, container: Json<NewContainer>) -> HttpResponse {
    let containers = &mut *containers.containers.lock().unwrap();

    connections.do_send(WsMessage(serde_json::to_string(&AddNewContainer::new(container.clone())).unwrap()));

    containers.push(Container { name: container.0.name });

    HttpResponse::Created().finish()
}
