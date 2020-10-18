use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    PlayerSelection,
    QuestionsAndAnswers,
    CharacterCreation,
    Game,
}

impl Default for GameState {
    fn default() -> Self {
        Self::PlayerSelection
    }
}
