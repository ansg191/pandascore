use serde::Deserialize;
use time::OffsetDateTime;

use crate::model::Winner;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
#[non_exhaustive]
pub struct CompactGame {
    /// The game begin time, UTC.
    /// `None` when the game status is [`GameStatus::NotStarted`].
    #[serde(with = "time::serde::iso8601::option")]
    pub begin_at: Option<OffsetDateTime>,
    /// Whether the game statistics are complete and won't be updated again.
    pub complete: bool,
    /// Whether historical data is available for the game.
    pub detailed_stats: bool,
    /// The game end time, UTC.
    /// `None` when the game status is not [`GameStatus::Finished`].
    #[serde(with = "time::serde::iso8601::option")]
    pub end_at: Option<OffsetDateTime>,
    /// Whether the game is finished.
    pub finished: bool,
    /// Whether the game has been forfeited.
    pub forfeit: bool,
    /// ID of the game.
    ///
    /// IDs are video game-specific, ie. a Valorant game and an Overwatch game can have the same
    /// game ID.
    pub id: u64,
    /// Duration of the game in seconds.
    ///
    /// `None` when the game status is not [`GameStatus::Finished`].
    pub length: Option<u64>,
    pub match_id: u64,
    /// Game position in the match. Starts at 1.
    pub position: u64,
    /// The game status
    pub status: GameStatus,
    pub winner: Option<Winner>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum GameStatus {
    Finished,
    NotPlayed,
    NotStarted,
    Running,
}
