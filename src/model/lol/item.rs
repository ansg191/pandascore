use compact_str::CompactString;
use serde::Deserialize;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct Item {
    pub flat_armor_mod: Option<u64>,
    pub flat_crit_chance_mod: Option<u64>,
    pub flat_hp_pool_mod: Option<u64>,
    pub flat_hp_regen_mod: Option<u64>,
    pub flat_magic_damage_mod: Option<u64>,
    pub flat_movement_speed_mod: Option<u64>,
    #[serde(rename = "flat_mp_pool_mod")]
    pub flat_mana_pool_mod: Option<u64>,
    #[serde(rename = "flat_mp_regen_mod")]
    pub flat_mana_regen_mod: Option<u64>,
    pub flat_physical_damage_mod: Option<u64>,
    #[serde(rename = "flat_spell_block_mod")]
    pub flat_magic_resist_mod: Option<u64>,
    pub gold_base: Option<u64>,
    pub gold_purchasable: Option<bool>,
    pub gold_sell: Option<u64>,
    pub gold_total: Option<u64>,
    pub id: u64,
    pub image_url: Option<String>,
    pub is_trinket: Option<bool>,
    pub name: CompactString,
    pub percent_attack_speed_mod: Option<u64>,
    pub percent_life_steal_mod: Option<u64>,
    pub percent_movement_speed_mod: Option<i64>,
    #[serde(rename = "videogame_versions")]
    pub video_game_versions: Vec<String>,
}
