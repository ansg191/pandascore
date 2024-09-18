use std::ops::{Deref, DerefMut};

use compact_str::CompactString;
use serde::Deserialize;
use time::OffsetDateTime;

use crate::model::{series::CompactSeries, VideoGame};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
#[non_exhaustive]
pub struct League {
    #[serde(flatten)]
    inner: CompactLeague,

    pub series: Vec<CompactSeries>,
    #[serde(rename = "videogame")]
    pub video_game: VideoGame,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
#[non_exhaustive]
pub struct CompactLeague {
    pub id: u64,
    pub image_url: Option<String>,
    #[serde(with = "time::serde::iso8601")]
    pub modified_at: OffsetDateTime,
    pub name: CompactString,
    pub slug: CompactString,
    pub url: Option<String>,
}

impl Deref for League {
    type Target = CompactLeague;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for League {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<CompactLeague> for League {
    fn as_ref(&self) -> &CompactLeague {
        &self.inner
    }
}

impl AsMut<CompactLeague> for League {
    fn as_mut(&mut self) -> &mut CompactLeague {
        &mut self.inner
    }
}

impl From<League> for CompactLeague {
    fn from(league: League) -> Self {
        league.inner
    }
}
