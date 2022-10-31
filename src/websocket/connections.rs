use std::collections::HashMap;
use actix::{Actor, Context, Handler, Recipient};
use uuid::Uuid;
use crate::websocket::local_message::{Connect, Disconnect};
use crate::websocket::ws_message::WsMessage;

pub struct WsConnections {
    pub users: HashMap<Uuid, Recipient<WsMessage>>,
}

impl Actor for WsConnections {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for WsConnections {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        self.users.remove(&msg.id);
    }
}

impl Handler<Connect> for WsConnections {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        self.users.insert(msg.uuid, msg.addr);
    }
}

impl Handler<WsMessage> for WsConnections {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.users.iter().for_each(|user| {
            user.1.do_send(WsMessage(msg.0.clone()));
        })
    }
}
