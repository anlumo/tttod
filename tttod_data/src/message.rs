use crate::{GameState, Player, PlayerStats};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum ClientToServerMessage {
    SetPlayerName(String),
    ReadyForGame,
    VoteKickPlayer(Uuid),
    RevertVoteKickPlayer(Uuid),
    Answers(Vec<String>),
    SetCharacter(PlayerStats),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum ServerToClientMessage {
    GameIsFull,
    GameIsOngoing,
    PushState {
        players: HashMap<Uuid, Player>,
        game_state: GameState,
        player_kick_votes: HashMap<Uuid, HashSet<Uuid>>,
    },
    Questions(Vec<(String, Option<String>)>),
    DeclareGM(Uuid),
    EndGame,
}

impl ClientToServerMessage {
    pub fn into_json(self) -> Result<String, serde_json::error::Error> {
        serde_json::to_string(&self)
    }
}

impl ServerToClientMessage {
    pub fn into_json(self) -> Result<String, serde_json::error::Error> {
        serde_json::to_string(&self)
    }
}
