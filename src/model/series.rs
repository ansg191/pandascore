use std::ops::{Deref, DerefMut};

use compact_str::CompactString;
use serde::Deserialize;
use time::OffsetDateTime;

use crate::model::{league::CompactLeague, tournament::CompactTournament, VideoGame, Winner};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct Series {
    #[serde(flatten)]
    inner: CompactSeries,

    pub league: CompactLeague,
    pub tournaments: Vec<CompactTournament>,
    #[serde(rename = "videogame")]
    pub video_game: VideoGame,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct CompactSeries {
    #[serde(with = "time::serde::iso8601::option")]
    pub begin_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::iso8601::option")]
    pub end_at: Option<OffsetDateTime>,
    pub full_name: CompactString,
    pub id: u64,
    pub league_id: u64,
    #[serde(with = "time::serde::iso8601")]
    pub modified_at: OffsetDateTime,
    pub name: Option<CompactString>,
    pub season: Option<CompactString>,
    pub slug: CompactString,
    #[serde(flatten)]
    pub winner: Option<Winner>,
    pub year: u16,
}

impl Deref for Series {
    type Target = CompactSeries;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Series {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<CompactSeries> for Series {
    fn as_ref(&self) -> &CompactSeries {
        &self.inner
    }
}

impl AsMut<CompactSeries> for Series {
    fn as_mut(&mut self) -> &mut CompactSeries {
        &mut self.inner
    }
}

impl From<Series> for CompactSeries {
    fn from(series: Series) -> Self {
        series.inner
    }
}
