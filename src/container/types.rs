use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::container::containers::{Container, ContainerState};
use crate::websocket::ws_message::WsMessage;

#[derive(Serialize, Deserialize)]
pub struct NewContainer {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateState {
    pub uuid: Uuid,
    pub state: ContainerState,
}

#[derive(Serialize)]
pub struct ClientUpdate<T: Serialize> {
    pub mode: String,
    pub data: T,
}

impl<T: Serialize> ClientUpdate<T> {
    pub fn new(mode: ClientUpdateMode, data: T) -> ClientUpdate<T> {
        ClientUpdate {
            mode: mode.value(),
            data,
        }
    }

    pub fn new_ws_message(mode: ClientUpdateMode, data: T) -> WsMessage {
        WsMessage(ClientUpdate::new(mode, data).to_string())
    }
}

impl<T: Serialize> ToString for ClientUpdate<T> {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

pub enum ClientUpdateMode {
    New,
    UpdateState,
}

impl ClientUpdateMode {
    pub fn value(&self) -> String {
        match self {
            ClientUpdateMode::New => "new".to_string(),
            ClientUpdateMode::UpdateState => "updateState".to_string()
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