use std::{collections::HashMap, time::Duration};

use actix::{Actor, AsyncContext, Context, Handler, Message, WeakAddr};
use serde::{Deserialize, Serialize};

use crate::{
    client::{FromClient, FromClientMessage, GameClient, ToClientMessage},
    state::GameState,
};

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Clone, Copy)]
pub struct ClientId(pub usize);

#[derive(Debug, Default)]
pub struct GameServer {
    state: GameState,
    clients: HashMap<ClientId, WeakAddr<GameClient>>,
}

impl Actor for GameServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_secs_f32(1.0 / 30.0), |_this, ctx| {
            ctx.notify(GameTick);
        });
    }
}

impl Handler<FromClient> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: FromClient, _ctx: &mut Self::Context) -> Self::Result {
        let resp = match msg.msg {
            FromClientMessage::NewPosition(x, y) => {
                self.state[msg.from].position = (x, y);
                ToClientMessage::Ack
            }
            FromClientMessage::Identify(id) => {
                self.state.add(id);
                self.clients.insert(id, msg.addr);
                ToClientMessage::Ack
            }
        };
        self.clients
            .get(&msg.from)
            .unwrap()
            .upgrade()
            .unwrap()
            .do_send(resp);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct GameTick;

impl Handler<GameTick> for GameServer {
    type Result = ();

    fn handle(&mut self, _msg: GameTick, _ctx: &mut Self::Context) -> Self::Result {
        self.clients.values().for_each(|c| {
            c.upgrade().map(|c| {
                c.do_send(ToClientMessage::State(self.state.clone()));
            });
        });
    }
}
