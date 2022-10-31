use std::collections::HashMap;
use std::time::{Duration, Instant};
use actix::{Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, Context, ContextFutureSpawner, fut, Handler, Recipient, Running, StreamHandler, WrapFuture};
use actix_web::{Error, get, HttpRequest, HttpResponse};
use actix_web::web::{Data, Payload};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message, ProtocolError, WebsocketContext};
use uuid::Uuid;
use crate::websocket::messages::{Connect, Disconnect, WsMessage};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[get("/ws")]
pub async fn start_connection(req: HttpRequest, stream: Payload, connections: Data<Addr<WebsocketConnections>>) -> Result<HttpResponse, Error> {
    let ws = WebsocketConnection::new(connections.get_ref().clone());

    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}

pub struct WebsocketConnections {
    pub users: HashMap<Uuid, Recipient<WsMessage>>,
}

impl Actor for WebsocketConnections {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for WebsocketConnections {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        self.users.remove(&msg.id);
    }
}

impl Handler<Connect> for WebsocketConnections {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        self.users.insert(msg.uuid, msg.addr);
    }
}

impl Handler<WsMessage> for WebsocketConnections {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.users.iter().for_each(|user| {
            user.1.do_send(WsMessage(msg.0.clone()));
        })
    }
}

pub struct WebsocketConnection {
    id: Uuid,
    connections: Addr<WebsocketConnections>,
    hb: Instant,
}

impl WebsocketConnection {
    pub fn new(websocket_connections: Addr<WebsocketConnections>) -> WebsocketConnection {
        WebsocketConnection {
            id: Uuid::new_v4(),
            connections: websocket_connections,
            hb: Instant::now(),
        }
    }

    fn hb(&self, ctx: &mut WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |connection, ctx| {
            if Instant::now().duration_since(connection.hb) > CLIENT_TIMEOUT {
                println!("user {} disconnected due heartbeat", connection.id.to_string());

                connection.connections.do_send(Disconnect {
                    id: connection.id,
                });

                ctx.stop();
                return;
            }

            ctx.ping(b"HELLO, PING!");
        });
    }
}

impl Actor for WebsocketConnection {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.connections
            .send(Connect {
                addr: addr.recipient(),
                uuid: self.id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => {}
                    _ => ctx.stop()
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.connections.do_send(Disconnect {
            id: self.id,
        });

        Running::Stop
    }
}

impl StreamHandler<Result<Message, ProtocolError>> for WebsocketConnection {
    fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(Message::Binary(bin)) => {
                ctx.binary(bin);
            }
            Ok(Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(Message::Nop) => {}
            Ok(Message::Text(s)) => {
                self.connections.do_send(WsMessage(s.to_string()))
            }
            _ => {}
        }
    }
}

impl Handler<WsMessage> for WebsocketConnection {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}