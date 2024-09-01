pub mod champions;
pub mod items;
pub mod matches;
pub mod series;
pub mod spells;
pub mod tournaments;

crate::endpoint::game_endpoints!("lol");
