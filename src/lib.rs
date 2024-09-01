//! # Rust client for the [PandaScore API](https://pandascore.co/).
//!
//! Currently **only** supports the free tier of the API.
//!
//! ## Features
//! - [ ] "All Video Games" endpoints
//!     - [ ] Incidents
//!     - [x] Leagues
//!     - [ ] Lives
//!     - [x] Matches
//!     - [x] Players
//!     - [ ] Series
//!     - [x] Teams
//!     - [ ] Tournaments
//!     - [ ] Video Games
//! - [ ] "League of Legends" endpoints
//!     - [x] Champions
//!     - [ ] ~Games~
//!     - [x] Items
//!     - [x] Leagues
//!     - [ ] ~Mastery~
//!     - [ ] ~Stats~
//!     - [x] Matches
//!     - [ ] ~Stats~
//!     - [x] Players
//!     - [x] Series
//!     - [x] Teams
//!     - [x] Spells
//!     - [x] Tournaments
//! - [ ] "Call of Duty" endpoints
//! - [ ] "Counter Strike" endpoints
//! - [ ] "Dota 2" endpoints
//! - [ ] "EA Sports FC" endpoints
//! - [ ] "LOL Wild Rift" endpoints
//! - [ ] "Mobile Legends: Bang Bang" endpoints
//! - [ ] "OverWatch" endpoints
//! - [ ] "PUBG" endpoints
//! - [ ] "Rainbow Six Siege" endpoints
//! - [ ] "Rocket League" endpoints
//! - [ ] "Valorant" endpoints
//! - [ ] "King of Glory" endpoints
//! - [ ] "StarCraft 2" endpoints
//! - [ ] "StarCraft Brood War" endpoints
//!
//! ## Examples
//! To search for a league by name:
//! ```rust,no_run
#![doc = include_str!("../examples/list_leagues.rs")]
//! ```
//! To get a player by ID or name:
//! ```rust,no_run
#![doc = include_str!("../examples/get_player.rs")]
//! ```

mod client;
pub mod endpoint;
pub mod model;

pub use client::Client;
