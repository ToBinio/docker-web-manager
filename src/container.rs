use std::io::Error;
use std::sync::Mutex;
use actix::{Actor, Addr, Context, Message, Handler, MailboxError};
use actix_web::{get, HttpResponse, post};
use actix_web::web::{Data, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::websocket::messages::{WsMessage};
use crate::websocket::websocket::WebsocketConnections;

pub struct Containers {
    pub containers: Mutex<Vec<Container>>,
    pub connections: Addr<WebsocketConnections>,
}

impl Actor for Containers {
    type Context = Context<Self>;
}

impl Handler<Add> for Containers {
    type Result = Result<Uuid, std::io::Error>;

    fn handle(&mut self, msg: Add, _ctx: &mut Self::Context) -> Self::Result {
        let uuid = Uuid::new_v4();

        self.containers.lock().unwrap().push(Container { name: msg.container.clone().name, uuid, updating: false });
        self.connections.do_send(WsMessage(serde_json::to_string(&AddNewContainer::new(msg.container.clone())).unwrap()));

        Ok(uuid)
    }
}

#[derive(Message)]
#[rtype(result = "Result<Uuid, std::io::Error>")]
pub struct Add {
    pub container: NewContainer,
}

impl Handler<GetALl> for Containers {
    type Result = Result<String, std::io::Error>;

    fn handle(&mut self, _msg: GetALl, _ctx: &mut Self::Context) -> Self::Result {
        let containers = &*self.containers.lock().unwrap();
        let string = serde_json::to_string(containers).unwrap();

        Ok(string)
    }
}

#[derive(Message)]
#[rtype(result = "Result<String, std::io::Error>")]
pub struct GetALl {}

#[derive(Serialize)]
pub struct Container {
    name: String,
    updating: bool,
    uuid: Uuid,
}

#[derive(Serialize, Deserialize)]
pub enum ContainerState {
    STATE1,
    STATE2,
    STATE3,
}

#[get("/containers")]
pub async fn get_all_containers(containers: Data<Addr<Containers>>) -> String {
    match containers.send(GetALl {}).await {
        Ok(res) => {
            println!("{:?}", res);
            return res.unwrap();
        }
        Err(_) => { return "[]".to_string(); }
    }
}

#[post("/container")]
pub async fn add_container(containers: Data<Addr<Containers>>, container: Json<NewContainer>) -> HttpResponse {
    match containers.send(Add {
        container: container.0
    }).await {
        Ok(_) => {}
        Err(_) => {}
    };

    HttpResponse::Created().finish()
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewContainer {
    pub name: String,
}

#[derive(Serialize)]
pub struct AddNewContainer {
    pub mode: String,
    pub data: NewContainer,
}

impl AddNewContainer {
    pub fn new(container: NewContainer) -> AddNewContainer {
        AddNewContainer {
            mode: "new".to_string(),
            data: container,
        }
    }
}
