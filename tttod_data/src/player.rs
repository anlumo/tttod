use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

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

impl fmt::Display for ArtifactBoon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reroll => write!(f, "Reroll"),
            Self::RollWithPlusTwo => write!(f, "Roll with +2 dice"),
            Self::SuccessOnFive => write!(f, "Success on 5+"),
            Self::SuccessOnDoubles => write!(f, "Success on doubles"),
        }
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

impl Condition {
    pub fn take_hit(self) -> Self {
        match self {
            Self::Hale => Self::Wounded,
            Self::Wounded => Self::Critical,
            _ => Self::Dead,
        }
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

impl MentalCondition {
    pub fn take_hit(self) -> Self {
        match self {
            Self::Hale => Self::Resisted,
            _ => Self::Possessed,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Attribute {
    Heroic,
    Booksmart,
    Streetwise,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub name: String,
    pub speciality: Speciality,
    pub reputation: Reputation,
    pub attributes: HashMap<Attribute, u8>,
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
            attributes: [
                (Attribute::Heroic, 3),
                (Attribute::Booksmart, 1),
                (Attribute::Streetwise, 1),
            ]
            .iter()
            .cloned()
            .collect(),
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

impl fmt::Display for Speciality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Religion => write!(f, "Religion"),
            Self::Linguistics => write!(f, "Linguistics"),
            Self::Architecture => write!(f, "Architecture"),
            Self::WarAndWeaponry => write!(f, "War and Weaponry"),
            Self::GemsAndMetals => write!(f, "Gems and Metals"),
            Self::SecretSignsSymbols => write!(f, "Secret Signs / Symbols"),
            Self::Osteology => write!(f, "Osteology"),
            Self::DeathAndBurial => write!(f, "Death and Burial"),
            Self::Other(other) => other.fmt(f),
        }
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

impl fmt::Display for Reputation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ambitious => write!(f, "Ambitious"),
            Self::Genius => write!(f, "Genius"),
            Self::Ruthless => write!(f, "Ruthless"),
            Self::Senile => write!(f, "Senile"),
            Self::MadScientist => write!(f, "Mad Scientist"),
            Self::BornLeader => write!(f, "Born Leader"),
            Self::Rulebreaker => write!(f, "Rulebreaker"),
            Self::Obsessive => write!(f, "Obsessive"),
            Self::Other(other) => other.fmt(f),
        }
    }
}
