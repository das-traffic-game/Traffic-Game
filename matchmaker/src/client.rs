use actix::{Actor, Addr, AsyncContext, Handler, Message, StreamHandler, WeakAddr};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};

use crate::{
    server::{ClientId, GameServer},
    state::GameState,
};

/// Define HTTP actor
pub struct GameClient {
    server: Addr<GameServer>,
    id: Option<ClientId>,
}

impl GameClient {
    pub fn new(addr: Addr<GameServer>) -> Self {
        Self {
            server: addr,
            id: None,
        }
    }
}

impl Actor for GameClient {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&ToClientMessage::Identify).unwrap());
    }
}

/// Handler for websocket packets
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameClient {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Binary(_)) => panic!("did not expect binary message"),
            Ok(ws::Message::Text(text)) => {
                let msg: FromClientMessage = serde_json::from_str(&text.to_string()).unwrap();

                match msg {
                    FromClientMessage::Identify(id) => {
                        self.id.replace(id);
                    }
                    _ => {}
                }

                self.server.do_send(FromClient {
                    msg,
                    addr: ctx.address().downgrade(),
                    from: self.id.unwrap(),
                });
            }
            _ => (),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FromClientMessage {
    NewPosition(f32, f32),
    Identify(ClientId),
}

#[derive(Debug, Serialize, Deserialize, Message)]
#[serde(rename_all = "snake_case")]
#[rtype(result = "()")]
pub enum ToClientMessage {
    Identify,
    Ack,
    Error(String),
    State(GameState),
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct FromClient {
    pub from: ClientId,
    pub addr: WeakAddr<GameClient>,
    pub msg: FromClientMessage,
}

impl Handler<ToClientMessage> for GameClient {
    type Result = ();

    fn handle(&mut self, msg: ToClientMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}
