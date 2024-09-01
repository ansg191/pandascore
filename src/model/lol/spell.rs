use compact_str::CompactString;
use serde::Deserialize;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct Spell {
    pub id: u64,
    pub image_url: String,
    pub name: CompactString,
}
