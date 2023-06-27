use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::server::ClientId;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub struct GameState {
    players: HashMap<ClientId, PlayerState>,
}

impl std::ops::Index<ClientId> for GameState {
    type Output = PlayerState;

    fn index(&self, index: ClientId) -> &Self::Output {
        &self.players[&index]
    }
}

impl std::ops::IndexMut<ClientId> for GameState {
    fn index_mut(&mut self, index: ClientId) -> &mut Self::Output {
        self.players.get_mut(&index).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct PlayerState {
    pub position: (f32, f32),
}

impl GameState {
    pub fn add(&mut self, id: ClientId) {
        self.players.insert(id, PlayerState::default());
    }
}
