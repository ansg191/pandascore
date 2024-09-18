use std::ops::{Deref, DerefMut};

use compact_str::CompactString;
use serde::Deserialize;
use time::OffsetDateTime;

use crate::model::{
    league::CompactLeague, player::CompactPlayer, series::CompactSeries, team::CompactTeam,
    tournament::CompactTournament, VideoGame, Winner,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
#[non_exhaustive]
pub struct Match {
    #[serde(flatten)]
    pub inner: CompactMatch,

    // pub games: Vec<_>
    pub league: CompactLeague,
    pub league_id: u64,
    // pub opponents: Vec<_>
    pub results: Vec<MatchResult>,
    pub serie: CompactSeries,
    pub serie_id: u64,
    pub tournament: CompactTournament,
    #[serde(rename = "videogame")]
    pub video_game: VideoGame,
    #[serde(rename = "videogame_version")]
    pub video_game_version: Option<MatchVideoGameVersion>,
    // pub winner: todo
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum MatchResult {
    Team { score: u32, team_id: u64 },
    Player { score: u32, player_id: u64 },
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
#[non_exhaustive]
pub struct MatchVideoGameVersion {
    /// Whether this videogame version is current
    pub current: bool,
    pub name: CompactString,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
#[non_exhaustive]
pub struct CompactMatch {
    #[serde(with = "time::serde::iso8601::option")]
    pub begin_at: Option<OffsetDateTime>,
    /// Whether the match offers full stats.
    pub detailed_stats: bool,
    /// Whether result of the match is a draw.
    pub draw: bool,
    #[serde(with = "time::serde::iso8601::option")]
    pub end_at: Option<OffsetDateTime>,
    /// Whether match was forfeited.
    pub forfeit: bool,
    /// ID of the opponent with a game advantage.
    pub game_advantage: Option<u64>,
    pub id: u64,
    pub live: MatchLive,
    pub match_type: MatchType,
    #[serde(with = "time::serde::iso8601")]
    pub modified_at: OffsetDateTime,
    pub name: CompactString,
    /// Number of games.
    pub number_of_games: u32,
    #[serde(with = "time::serde::iso8601::option")]
    pub original_scheduled_at: Option<OffsetDateTime>,
    /// Whether match has been rescheduled.
    pub rescheduled: Option<bool>,
    #[serde(with = "time::serde::iso8601::option")]
    pub scheduled_at: Option<OffsetDateTime>,
    pub slug: String,
    pub status: MatchStatus,
    // pub streams_list: Vec<_>,
    pub tournament_id: u64,
    #[serde(flatten)]
    pub winner: Option<Winner>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MatchType {
    AllGamesPlayed,
    BestOf,
    Custom,
    FirstTo,
    OwBestOf,
    RedBullHomeGround,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MatchStatus {
    Canceled,
    Finished,
    NotStarted,
    Postponed,
    Running,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
#[non_exhaustive]
pub struct MatchLive {
    #[serde(with = "time::serde::iso8601::option")]
    pub opens_at: Option<OffsetDateTime>,
    /// Whether live is supported
    pub supported: bool,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
#[serde(tag = "type", content = "opponent")]
#[non_exhaustive]
pub enum CompactMatchOpponent {
    Team(CompactTeam),
    Player(CompactPlayer),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
#[serde(tag = "opponent_type", content = "opponents")]
#[non_exhaustive]
pub enum MatchOpponents {
    Team(Vec<CompactTeam>),
    Player(Vec<CompactPlayer>),
}

impl Deref for Match {
    type Target = CompactMatch;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Match {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<CompactMatch> for Match {
    fn as_ref(&self) -> &CompactMatch {
        &self.inner
    }
}

impl AsMut<CompactMatch> for Match {
    fn as_mut(&mut self) -> &mut CompactMatch {
        &mut self.inner
    }
}

impl From<Match> for CompactMatch {
    fn from(series: Match) -> Self {
        series.inner
    }
}
