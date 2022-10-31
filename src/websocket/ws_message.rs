use actix::Message;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);