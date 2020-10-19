use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum GameState {
    PlayerSelection,
    DefineEvil,
    CharacterCreation,
    CharacterIntroduction,
    Room(usize),
    FinalBattle,
}

impl Default for GameState {
    fn default() -> Self {
        Self::PlayerSelection
    }
}
