use crate::Attribute;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Challenge {
    pub player_id: Uuid,
    pub attribute: Attribute,
    pub speciality_applies: bool,
    pub reputation_applies: bool,
}
