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
pub enum ArtifactBoon {
    Reroll,
    RollWithPlusTwo,
    SuccessOnFive,
    SuccessOnDoubles,
}

impl Default for ArtifactBoon {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub name: String,
    pub speciality: Speciality,
    pub reputation: Reputation,
    pub heroic: u8,
    pub booksmart: u8,
    pub streetwise: u8,
    pub artifact_name: String,
    pub artifact_origin: String,
    pub artifact_boon: ArtifactBoon,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            speciality: Speciality::default(),
            reputation: Reputation::default(),
            heroic: 3,
            booksmart: 1,
            streetwise: 1,
            artifact_name: "".to_owned(),
            artifact_origin: "".to_owned(),
            artifact_boon: ArtifactBoon::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Speciality {
    Religion,
    Linguistics,
    Architecture,
    WarAndWeaponry,
    GemsAndMetals,
    SecretSignsSymbols,
    Osteology,
    DeathAndBurial,
    Other(String),
}

impl Default for Speciality {
    fn default() -> Self {
        Self::Religion
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Reputation {
    Ambitious,
    Genius,
    Ruthless,
    Senile,
    MadScientist,
    BornLeader,
    Rulebreaker,
    Obsessive,
    Other(String),
}

impl Default for Reputation {
    fn default() -> Self {
        Self::Ambitious
    }
}
