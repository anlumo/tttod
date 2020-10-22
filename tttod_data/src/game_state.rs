use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum GameState {
    PlayerSelection,
    DefineEvil,
    CharacterCreation,
    CharacterIntroduction,
    Room {
        room_idx: usize,
        gm: Uuid,
        successes: usize,
        failures: usize,
    },
    FinalBattle {
        remaining_clues: Vec<String>,
        gms: HashSet<Uuid>,
        successes: usize,
        target_successes: usize,
    },
    Victory,
    Failure,
}

impl Default for GameState {
    fn default() -> Self {
        Self::PlayerSelection
    }
}

pub const SUCCESSES_NEEDED: usize = 3;
pub const FAILURES_NEEDED: usize = 3;
