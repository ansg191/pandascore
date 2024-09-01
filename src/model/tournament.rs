use std::ops::{Deref, DerefMut};

use compact_str::CompactString;
use serde::Deserialize;
use time::OffsetDateTime;

use crate::model::{
    league::CompactLeague, matches::CompactMatch, player::CompactPlayer, series::CompactSeries,
    team::CompactTeam, VideoGame, Winner,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct Tournament {
    #[serde(flatten)]
    inner: CompactTournament,

    #[serde(default)]
    pub expected_roster: Vec<Roster>,
    pub league: CompactLeague,
    pub matches: Vec<CompactMatch>,
    pub serie: CompactSeries,
    pub teams: Vec<CompactTeam>,
    #[serde(rename = "videogame")]
    pub video_game: VideoGame,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct Roster {
    pub players: Vec<CompactPlayer>,
    pub team: CompactTeam,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct CompactTournament {
    #[serde(with = "time::serde::iso8601::option")]
    pub begin_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::iso8601::option")]
    pub end_at: Option<OffsetDateTime>,
    pub detailed_stats: bool,
    pub has_bracket: bool,
    pub id: u64,
    pub league_id: u64,
    pub live_supported: bool,
    #[serde(with = "time::serde::iso8601")]
    pub modified_at: OffsetDateTime,
    pub name: CompactString,
    #[serde(rename = "prizepool")]
    pub prize_pool: Option<CompactString>,
    pub serie_id: u64,
    pub slug: CompactString,
    pub tier: Option<Tier>,
    #[serde(flatten)]
    pub winner: Option<Winner>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    Unranked,
    D,
    C,
    B,
    A,
    S,
}

impl Deref for Tournament {
    type Target = CompactTournament;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Tournament {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<CompactTournament> for Tournament {
    fn as_ref(&self) -> &CompactTournament {
        &self.inner
    }
}

impl AsMut<CompactTournament> for Tournament {
    fn as_mut(&mut self) -> &mut CompactTournament {
        &mut self.inner
    }
}

impl From<Tournament> for CompactTournament {
    fn from(series: Tournament) -> Self {
        series.inner
    }
}
