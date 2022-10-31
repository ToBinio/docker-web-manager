use actix::{Addr, Message, Recipient};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub uuid: Uuid,
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
