use std::fmt::Display;

use compact_str::CompactString;
use serde::Deserialize;

pub mod bracket;
pub mod game;
pub mod league;
pub mod lol;
pub mod matches;
pub mod player;
pub mod series;
pub mod team;
pub mod tournament;
mod winner;

pub use winner::Winner;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct VideoGame {
    pub id: u64,
    pub name: CompactString,
    pub slug: CompactString,
    pub current_version: Option<CompactString>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize)]
pub enum Identifier<'a> {
    Id(u64),
    Slug(&'a str),
}

impl From<u64> for Identifier<'_> {
    fn from(id: u64) -> Self {
        Self::Id(id)
    }
}
impl<'a> From<&'a str> for Identifier<'a> {
    fn from(slug: &'a str) -> Self {
        Self::Slug(slug)
    }
}

impl Display for Identifier<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id(id) => write!(f, "{}", id),
            Self::Slug(slug) => write!(f, "{}", slug),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum EventStatus {
    Past,
    Running,
    Upcoming,
}

impl EventStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Past => "past",
            Self::Running => "running",
            Self::Upcoming => "upcoming",
        }
    }
}
