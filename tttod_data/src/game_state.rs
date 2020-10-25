use crate::Challenge;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum GameState {
    PlayerSelection {
        player_kick_votes: HashMap<Uuid, HashSet<Uuid>>,
    },
    DefineEvil,
    CharacterCreation,
    CharacterIntroduction,
    Room {
        room_idx: usize,
        gm: Uuid,
        successes: usize,
        failures: usize,
        challenge: Option<Challenge>,
        known_clues: Vec<String>,
    },
    FinalBattle {
        remaining_clues: Vec<String>,
        gms: HashSet<Uuid>,
        successes: usize,
        target_successes: usize,
        challenge: Option<Challenge>,
        chosen_clue: Option<usize>,
    },
    Victory,
    Failure,
}

impl Default for GameState {
    fn default() -> Self {
        Self::PlayerSelection {
            player_kick_votes: HashMap::new(),
        }
    }
}

pub const SUCCESSES_NEEDED: usize = 3;
pub const FAILURES_NEEDED: usize = 3;
