use std::io::Error;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use actix::{Actor, Addr, Context, Message, Handler};
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

#[derive(Message)]
#[rtype(result = "Result<Uuid, std::io::Error>")]
pub struct Add {
    pub container: NewContainer,
}

impl Handler<Add> for Containers {
    type Result = Result<Uuid, Error>;

    fn handle(&mut self, msg: Add, _ctx: &mut Self::Context) -> Self::Result {
        let uuid = Uuid::new_v4();

        let mut container = Container { name: msg.container.name, uuid, updating_thread: None, state_channel: channel(), state: ContainerState::STATE1 };

        self.connections.do_send(WsMessage(ClientWsUpdate::new(ClientWsUpdateMode::New, ClientContainer::from_container(&mut container)).to_string()));

        self.containers.lock().unwrap().push(container);

        Ok(uuid)
    }
}

#[derive(Message)]
#[rtype(result = "Result<String, std::io::Error>")]
pub struct GetALl {}

impl Handler<GetALl> for Containers {
    type Result = Result<String, Error>;

    fn handle(&mut self, _msg: GetALl, _ctx: &mut Self::Context) -> Self::Result {
        let containers = &*self.containers
            .lock()
            .unwrap()
            .iter_mut()
            .map(ClientContainer::from_container)
            .collect::<Vec<ClientContainer>>();
        let string = serde_json::to_string(containers).unwrap();

        Ok(string)
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Update {
    uuid: Uuid,
}

impl Handler<Update> for Containers {
    type Result = ();

    fn handle(&mut self, msg: Update, _ctx: &mut Self::Context) -> Self::Result {
        let containers = &mut *self.containers.lock().unwrap();
        let container = containers.iter_mut().find(|container| container.uuid == msg.uuid).unwrap();

        if container.updating_thread.is_some() && container.updating_thread.as_ref().unwrap().is_finished() {
            return;
        }

        let uuid = container.uuid;
        let connections = self.connections.clone();
        let tx = container.state_channel.0.clone();

        container.updating_thread = Some(thread::spawn(move || {
            Container::send_state_update(&tx, &connections, ContainerState::STATE1, uuid);

            thread::sleep(Duration::from_secs(5));
            Container::send_state_update(&tx, &connections, ContainerState::STATE2, uuid);

            thread::sleep(Duration::from_secs(5));
            Container::send_state_update(&tx, &connections, ContainerState::STATE3, uuid);
        }));
    }
}

pub struct Container {
    name: String,
    updating_thread: Option<JoinHandle<()>>,
    uuid: Uuid,
    state_channel: (Sender<ContainerState>, Receiver<ContainerState>),
    state: ContainerState,
}

impl Container {
    fn send_state_update(tx: &Sender<ContainerState>, connections: &Addr<WebsocketConnections>, new_state: ContainerState, uuid: Uuid) {
        tx.send(new_state.clone()).expect("Unable to send on channel");
        connections.do_send(WsMessage(ClientWsUpdate::new(ClientWsUpdateMode::UpdateState, UpdateContainerState {
            uuid,
            state: new_state,
        }).to_string()));
    }

    fn get_state(&mut self) -> ContainerState {
        for new_state in self.state_channel.1.try_iter()
        {
            self.state = new_state;
        }

        self.state.clone()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ContainerState {
    STATE1,
    STATE2,
    STATE3,
}

#[get("/containers")]
pub async fn get_all_containers(containers: Data<Addr<Containers>>) -> String {
    match containers.send(GetALl {}).await {
        Ok(res) => {
            res.unwrap()
        }
        Err(_) => "[]".to_string()
    }
}

#[post("/container")]
pub async fn add_container(containers: Data<Addr<Containers>>, container: Json<NewContainer>) -> HttpResponse {
    match containers.send(Add {
        container: container.0
    }).await {
        Ok(uuid) => {
            containers.do_send(Update {
                uuid: uuid.unwrap()
            })
        }
        Err(_) => { return HttpResponse::InternalServerError().finish(); }
    };

    HttpResponse::Created().finish()
}

#[derive(Serialize, Deserialize)]
pub struct NewContainer {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateContainerState {
    pub uuid: Uuid,
    pub state: ContainerState,
}

#[derive(Serialize)]
pub struct ClientWsUpdate<T: Serialize> {
    pub mode: String,
    pub data: T,
}

impl<T: Serialize> ClientWsUpdate<T> {
    pub fn new(mode: ClientWsUpdateMode, data: T) -> ClientWsUpdate<T> {
        ClientWsUpdate {
            mode: mode.value(),
            data,
        }
    }
}

impl<T: Serialize> ToString for ClientWsUpdate<T> {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

pub enum ClientWsUpdateMode {
    New,
    UpdateState,
}

impl ClientWsUpdateMode {
    pub fn value(&self) -> String {
        match self {
            ClientWsUpdateMode::New => "new".to_string(),
            ClientWsUpdateMode::UpdateState => "updateState".to_string()
        }
    }
}

#[derive(Serialize)]
pub struct ClientContainer {
    pub name: String,
    pub uuid: Uuid,
    pub state: ContainerState,
}

impl ClientContainer {
    pub fn from_container(container: &mut Container) -> ClientContainer {
        ClientContainer {
            name: container.name.clone(),
            uuid: container.uuid,
            state: container.get_state(),
        }
    }
}

