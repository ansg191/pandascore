use std::ops::{Deref, DerefMut};

use compact_str::CompactString;
use serde::Deserialize;
use time::OffsetDateTime;

use crate::model::{player::CompactPlayer, VideoGame};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct Team {
    #[serde(flatten)]
    pub inner: CompactTeam,

    #[serde(rename = "current_videogame")]
    pub current_video_game: Option<VideoGame>,
    pub players: Vec<CompactPlayer>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct CompactTeam {
    pub acronym: Option<CompactString>,
    /// The ID of the team.
    pub id: u64,
    /// URL of the team logo
    pub image_url: Option<String>,
    /// The team's organization location.
    pub location: Option<CompactString>,
    #[serde(with = "time::serde::iso8601")]
    pub modified_at: OffsetDateTime,
    /// The name of the team.
    pub name: CompactString,
    pub slug: Option<CompactString>,
}

impl Deref for Team {
    type Target = CompactTeam;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Team {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<CompactTeam> for Team {
    fn as_ref(&self) -> &CompactTeam {
        &self.inner
    }
}

impl AsMut<CompactTeam> for Team {
    fn as_mut(&mut self) -> &mut CompactTeam {
        &mut self.inner
    }
}

impl From<Team> for CompactTeam {
    fn from(team: Team) -> Self {
        team.inner
    }
}
