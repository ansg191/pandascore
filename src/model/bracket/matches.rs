use std::fmt::{Display, Formatter};

use serde::Deserialize;

use crate::model::matches::{CompactMatch, CompactMatchOpponent, MatchResult};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct TournamentBracketMatch {
    #[serde(flatten)]
    pub inner: CompactMatch,

    // pub games: Vec<BracketGame>,
    pub opponents: Vec<CompactMatchOpponent>,
    pub previous_matches: Vec<TournamentPreviousMatch>,
    pub results: Vec<MatchResult>,
}

impl Display for TournamentBracketMatch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.name)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct TournamentPreviousMatch {
    pub match_id: u64,
    #[serde(rename = "type")]
    pub r#type: PreviousMatchType,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviousMatchType {
    Winner,
    Loser,
}

impl Display for PreviousMatchType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Winner => write!(f, "winner"),
            Self::Loser => write!(f, "loser"),
        }
    }
}
