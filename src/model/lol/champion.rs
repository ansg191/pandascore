use compact_str::CompactString;
use serde::Deserialize;
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Champion {
    pub armor: f64,
    #[serde(rename = "armorperlevel")]
    pub armor_per_level: f64,
    #[serde(rename = "attackdamage")]
    pub attack_damage: f64,
    #[serde(rename = "attackdamageperlevel")]
    pub attack_damage_per_level: f64,
    #[serde(rename = "attackrange")]
    pub attack_range: f64,
    #[serde(rename = "attackspeedoffset")]
    pub attack_speed_offset: Option<f64>,
    #[serde(rename = "attackspeedperlevel")]
    pub attack_speed_per_level: f64,
    pub big_image_url: String,
    pub crit: f64,
    #[serde(rename = "critperlevel")]
    pub crit_per_level: f64,
    pub hp: f64,
    #[serde(rename = "hpperlevel")]
    pub hp_per_level: f64,
    #[serde(rename = "hpregen")]
    pub hp_regen: f64,
    #[serde(rename = "hpregenperlevel")]
    pub hp_regen_per_level: f64,
    pub id: u64,
    pub image_url: String,
    #[serde(rename = "movespeed")]
    pub move_speed: f64,
    #[serde(rename = "mp")]
    pub mana: f64,
    #[serde(rename = "mpperlevel")]
    pub mana_per_level: f64,
    #[serde(rename = "mpregen")]
    pub mana_regen: f64,
    #[serde(rename = "mpregenperlevel")]
    pub mana_regen_per_level: f64,
    pub name: CompactString,
    #[serde(rename = "spellblock")]
    pub magic_resist: i64,
    #[serde(rename = "spellblockperlevel")]
    pub magic_resist_per_level: f64,
    #[serde(rename = "videogame_versions")]
    pub video_game_versions: Vec<CompactString>,
}
