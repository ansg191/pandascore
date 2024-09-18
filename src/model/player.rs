use std::ops::{Deref, DerefMut};

use compact_str::CompactString;
use serde::Deserialize;
use time::{Date, OffsetDateTime};

use crate::model::{team::CompactTeam, VideoGame};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct Player {
    #[serde(flatten)]
    pub inner: CompactPlayer,

    pub current_team: Option<CompactTeam>,
    #[serde(rename = "current_videogame")]
    pub current_video_game: Option<VideoGame>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct CompactPlayer {
    /// Whether player is active.
    pub active: bool,
    /// Age of the player, null if unknown. When `birthday` is null, age is an approxiamation.
    /// Note: this field is only present for users running the Historical plan or up.
    pub age: Option<u8>,
    /// Birthday of the player.
    /// Note: this field is only present for users running the Historical plan or up.
    #[serde(with = "birthday_format", default)]
    pub birthday: Option<Date>,
    /// First name of the player.
    pub first_name: Option<CompactString>,
    /// ID of the player.
    pub id: u64,
    /// URL to the photo of the player.
    pub image_url: Option<String>,
    /// Last name of the player.
    pub last_name: Option<CompactString>,
    #[serde(with = "time::serde::iso8601")]
    pub modified_at: OffsetDateTime,
    /// Professional name of the player.
    pub name: String,
    /// Country code matching the nationality of the player according to the ISO 3166-1 standard (Alpha-2 code).
    /// In addition to the standard, the XK code is used for Kosovo.
    pub nationality: Option<CompactString>,
    /// Role/position of the player. Field value varies depending on the video game.
    /// Note: role is only available for DOTA 2, League of Legends, and Overwatch players.
    pub role: Option<CompactString>,
    /// Unique, human-readable identifier for the player.
    /// `id` and `slug` can be used interchangeably throughout the API.
    pub slug: Option<CompactString>,
}

/// Equivalent to below, but works when `null` is present in the JSON.
/// ```
/// time::serde::format_description!(birthday_format, Date, "[year]-[month]-[day]");
/// ```
mod birthday_format {
    use serde::Deserialize;
    use time::{format_description::BorrowedFormatItem, Date};

    const FORMAT: &[BorrowedFormatItem<'static>] =
        time::macros::format_description!("[year]-[month]-[day]");

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Date>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: Option<&'de str> = Deserialize::deserialize(deserializer)?;
        s.map(|s| Date::parse(s, FORMAT).map_err(serde::de::Error::custom))
            .transpose()
    }
}

impl Deref for Player {
    type Target = CompactPlayer;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Player {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<CompactPlayer> for Player {
    fn as_ref(&self) -> &CompactPlayer {
        &self.inner
    }
}

impl AsMut<CompactPlayer> for Player {
    fn as_mut(&mut self) -> &mut CompactPlayer {
        &mut self.inner
    }
}

impl From<Player> for CompactPlayer {
    fn from(player: Player) -> Self {
        player.inner
    }
}
