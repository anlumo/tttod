use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Player {
    pub ready: bool,
    pub name: String,
    pub stats: Option<PlayerStats>,
    pub condition: Condition,
    pub mental_condition: MentalCondition,
    pub artifact_used: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactPower {
    Reroll,
    RollwithPlusTwo,
    SuccessOnFive,
    SuccessOnDoubles,
}

impl Default for ArtifactPower {
    fn default() -> Self {
        Self::Reroll
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Condition {
    Hale,
    Wounded,
    Critical,
    Dead,
}

impl Default for Condition {
    fn default() -> Self {
        Self::Hale
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MentalCondition {
    Hale,
    Resisted,
    Possessed,
}

impl Default for MentalCondition {
    fn default() -> Self {
        Self::Hale
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerStats {
    pub name: String,
    pub speciality: String,
    pub reputation: String,
    pub heroic: u8,
    pub booksmart: u8,
    pub streetwise: u8,
    pub artifact_name: String,
    pub artifact_origin: String,
    pub artifact_power: ArtifactPower,
}
