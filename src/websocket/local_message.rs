use actix::{Message, Recipient};
use uuid::Uuid;
use crate::websocket::ws_message::WsMessage;

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