use std::fmt::Formatter;

use serde::{
    de::{Error, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_json::Value;

use crate::model::{player::CompactPlayer, team::CompactTeam};

/// Winner object
///
/// The `PandaScore` API returns multiple ways to represent the winner of various events.
///
/// The various ways to represent the winner are:
/// ```json
/// {
///     "winner_id": 1,
///     "winner_type": "Team or Player"
/// }
/// ```
/// or
/// ```json
/// {
///     "winner": <CompactTeamObject or CompactPlayerObject>,
///     "winner_id": 1,
///     "winner_type": "Team or Player"
/// }
/// ```
/// or
/// ```json
/// {
///     "winner": {
///         "id": 1
///         "type": "Team or Player"
///     },
///     "winner_type": "Team or Player"
/// }
/// ```
///
/// The `Winner` object is a Rust enum that can represent all of these cases
/// and attempts to deserialize the various representations into a single enum.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Winner {
    Team {
        /// The team ID that won the event.
        id: Option<u64>,
        /// The team that won the event.
        /// Boxed to reduce size.
        /// This is only present in Match objects.
        team: Option<Box<CompactTeam>>,
    },
    Player {
        /// The player ID that won the event.
        id: Option<u64>,
        /// The player that won the event.
        /// Boxed to reduce size.
        /// This is only present in Match objects.
        player: Option<Box<CompactPlayer>>,
    },
}

impl<'de> Deserialize<'de> for Winner {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(WinnerVisitor)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum WinnerFields {
    Winner,
    WinnerId,
    WinnerType,
}

struct WinnerVisitor;

impl<'de> Visitor<'de> for WinnerVisitor {
    type Value = Winner;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("enum Winner")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut tp: Option<WinnerType> = None;
        let mut id: Option<u64> = None;
        let mut obj: Option<Value> = None;
        while let Some(key) = map.next_key()? {
            match key {
                WinnerFields::Winner => {
                    if obj.is_some() {
                        return Err(Error::duplicate_field("winner"));
                    }
                    obj = Some(map.next_value()?);
                }
                WinnerFields::WinnerId => {
                    if id.is_some() {
                        return Err(Error::duplicate_field("winner_id"));
                    }
                    id = map.next_value()?;
                }
                WinnerFields::WinnerType => {
                    if tp.is_some() {
                        return Err(Error::duplicate_field("winner_type"));
                    }
                    tp = map.next_value()?;
                }
            }
        }
        match obj {
            None => case1::<A>(tp, id),
            Some(obj) if obj.is_null() => case1::<A>(tp, id),
            Some(obj) => {
                // Either case 2 or 3
                // Case 3
                let winner = serde_json::from_value::<WinnerInternal>(obj.clone());
                winner.map_or_else(
                    |_| case2::<A>(tp, id, obj),
                    |w| match w.r#type {
                        WinnerType::Team => Ok(Winner::Team {
                            id: Some(w.id),
                            team: None,
                        }),
                        WinnerType::Player => Ok(Winner::Player {
                            id: Some(w.id),
                            player: None,
                        }),
                    },
                )
            }
        }
    }
}

fn case1<'de, A>(tp: Option<WinnerType>, id: Option<u64>) -> Result<Winner, A::Error>
where
    A: MapAccess<'de>,
{
    match tp.ok_or_else(|| Error::missing_field("winner_type"))? {
        WinnerType::Team => Ok(Winner::Team { id, team: None }),
        WinnerType::Player => Ok(Winner::Player { id, player: None }),
    }
}

fn case2<'de, A>(tp: Option<WinnerType>, id: Option<u64>, obj: Value) -> Result<Winner, A::Error>
where
    A: MapAccess<'de>,
{
    match tp.ok_or_else(|| Error::missing_field("winner_type"))? {
        WinnerType::Team => Ok(Winner::Team {
            id,
            team: Some(Box::new(
                serde_json::from_value(obj).map_err(Error::custom)?,
            )),
        }),
        WinnerType::Player => Ok(Winner::Player {
            id,
            player: Some(Box::new(
                serde_json::from_value(obj).map_err(Error::custom)?,
            )),
        }),
    }
}

#[derive(Debug, Deserialize)]
struct WinnerInternal {
    id: u64,
    #[serde(rename = "type")]
    r#type: WinnerType,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize)]
enum WinnerType {
    Team,
    Player,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Deserialize)]
    struct TestStruct {
        extra: i32,
        #[serde(flatten)]
        winner: Winner,
    }

    #[derive(Debug, PartialEq, Eq, Deserialize)]
    struct TestStructOption {
        extra: i32,
        #[serde(flatten)]
        winner: Option<Winner>,
    }

    #[test]
    fn test_winner_case1() {
        let json = r#"{"extra":42,"winner_id":1,"winner_type":"Team"}"#;
        let test = TestStruct {
            extra: 42,
            winner: Winner::Team {
                id: Some(1),
                team: None,
            },
        };
        assert_eq!(serde_json::from_str::<TestStruct>(json).unwrap(), test);

        let json = r#"{"extra":42,"winner_id":1,"winner_type":"Player"}"#;
        let test = TestStructOption {
            extra: 42,
            winner: Some(Winner::Player {
                id: Some(1),
                player: None,
            }),
        };
        assert_eq!(
            serde_json::from_str::<TestStructOption>(json).unwrap(),
            test
        );

        let json = r#"{"extra":42,"winner_id":null,"winner_type":null}"#;
        let test = TestStructOption {
            extra: 42,
            winner: None,
        };
        assert_eq!(
            serde_json::from_str::<TestStructOption>(json).unwrap(),
            test
        );

        let json = r#"{"extra":42,"winner_id":null,"winner_type":"Team"}"#;
        let test = TestStructOption {
            extra: 42,
            winner: Some(Winner::Team {
                id: None,
                team: None,
            }),
        };
        assert_eq!(
            serde_json::from_str::<TestStructOption>(json).unwrap(),
            test
        );
    }

    #[test]
    fn test_winner_case2() {
        let json = r#"{
    "extra": 42,
    "winner": {
        "modified_at": "1970-01-01T00:00:00Z",
        "id": 1,
        "name": "T1"
    },
    "winner_id": 1,
    "winner_type": "Team"
}"#;
        let test = TestStruct {
            extra: 42,
            winner: Winner::Team {
                id: Some(1),
                team: Some(Box::new(CompactTeam {
                    acronym: None,
                    id: 1,
                    image_url: None,
                    location: None,
                    modified_at: time::OffsetDateTime::from_unix_timestamp(0).unwrap(),
                    name: "T1".into(),
                    slug: None,
                })),
            },
        };
        assert_eq!(serde_json::from_str::<TestStruct>(json).unwrap(), test);

        let json = r#"{
    "extra": 42,
    "winner": null,
    "winner_id": 1,
    "winner_type": "Team"
}"#;
        let test = TestStruct {
            extra: 42,
            winner: Winner::Team {
                id: Some(1),
                team: None,
            },
        };
        assert_eq!(serde_json::from_str::<TestStruct>(json).unwrap(), test);
    }

    #[test]
    fn test_winner_case3() {
        let json = r#"{"extra":42,"winner":{"id":1,"type":"Player"}}"#;
        let test = TestStruct {
            extra: 42,
            winner: Winner::Player {
                id: Some(1),
                player: None,
            },
        };
        assert_eq!(serde_json::from_str::<TestStruct>(json).unwrap(), test);

        let json = r#"{"extra":42,"winner":{"id":1,"type":"Player"}}"#;
        let test = TestStructOption {
            extra: 42,
            winner: Some(Winner::Player {
                id: Some(1),
                player: None,
            }),
        };
        assert_eq!(
            serde_json::from_str::<TestStructOption>(json).unwrap(),
            test
        );

        let json = r#"{"extra":42,"winner":null}"#;
        let test = TestStructOption {
            extra: 42,
            winner: None,
        };
        assert_eq!(
            serde_json::from_str::<TestStructOption>(json).unwrap(),
            test
        );

        let json = r#"{"extra":42,"winner":null,"winner_type":"Team"}"#;
        let test = TestStructOption {
            extra: 42,
            winner: Some(Winner::Team {
                id: None,
                team: None,
            }),
        };
        assert_eq!(
            serde_json::from_str::<TestStructOption>(json).unwrap(),
            test
        );
    }
}
