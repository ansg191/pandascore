//! # League of Legends Endpoints

pub mod champions;
pub mod items;
pub mod spells;

crate::endpoint::game_endpoints!("lol");
