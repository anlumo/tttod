use crate::{GameState, Player};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageFrame {
    pub id: Uuid,
    #[serde(flatten)]
    pub msg: ClientToServerMessage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum ClientToServerMessage {
    SetPlayerName(String),
    ReadyForGame,
    VoteKickPlayer(Uuid),
    RevertVoteKickPlayer(Uuid),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum ServerToClientMessage {
    PushState {
        players: HashMap<Uuid, Player>,
        game_state: GameState,
        player_kick_votes: HashMap<Uuid, HashSet<Uuid>>,
    },
    EndGame,
}

impl ClientToServerMessage {
    pub fn into_json(self) -> Result<String, serde_json::error::Error> {
        serde_json::to_string(&MessageFrame {
            id: Uuid::new_v4(),
            msg: self,
        })
    }
}

impl ServerToClientMessage {
    pub fn into_json(self) -> Result<String, serde_json::error::Error> {
        serde_json::to_string(&self)
    }
}
