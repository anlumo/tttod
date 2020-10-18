use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Question {
    SourceOfPower,
    Weakness,
    Intention,
    Creation,
    DefeatEnemies,
    MostTerrifying,
    Motivation,
    KeptSealed,
    TrueForm,
    Temptation,
}

impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceOfPower => write!(f, "What is the source of my power?"),
            Self::Weakness => write!(f, "What is my greatest weakness and why?"),
            Self::Intention => write!(
                f,
                "What do I intend to do with the world once I conquer it?"
            ),
            Self::Creation => write!(f, "What created me and how?"),
            Self::DefeatEnemies => write!(f, "How do I defeat my enemies?"),
            Self::MostTerrifying => write!(f, "What is most terrifying about me and why?"),
            Self::Motivation => write!(f, "What motivates me and drives me forward?"),
            Self::KeptSealed => write!(f, "What kept me sealed away all these years?"),
            Self::TrueForm => write!(f, "What does my true form look like?"),
            Self::Temptation => write!(f, "What do I promise to temp others to obey me?"),
        }
    }
}
