# pandascore ![License: MIT](https://img.shields.io/badge/license-MIT-blue) [![pandascore on crates.io](https://img.shields.io/crates/v/pandascore)](https://crates.io/crates/pandascore) [![pandascore on docs.rs](https://docs.rs/pandascore/badge.svg)](https://docs.rs/pandascore) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/ansg191/pandascore)

## Rust client for the [PandaScore API][__link0].

Currently **only** supports the free tier of the API.

### Features

* [ ] “All Video Games” endpoints
  * [ ] Incidents
  * [x] Leagues
  * [ ] Lives
  * [x] Matches
  * [x] Players
  * [ ] Series
  * [x] Teams
  * [ ] Tournaments
  * [ ] Video Games
* [ ] “League of Legends” endpoints
  * [x] Champions
  * [ ] ~~Games~~
  * [x] Items
  * [x] Leagues
  * [ ] ~~Mastery~~
  * [ ] ~~Stats~~
  * [x] Matches
  * [ ] ~~Stats~~
  * [x] Players
  * [x] Series
  * [x] Teams
  * [x] Spells
  * [x] Tournaments
* [ ] “Call of Duty” endpoints
* [ ] “Counter Strike” endpoints
* [ ] “Dota 2” endpoints
* [ ] “EA Sports FC” endpoints
* [ ] “LOL Wild Rift” endpoints
* [ ] “Mobile Legends: Bang Bang” endpoints
* [ ] “OverWatch” endpoints
* [ ] “PUBG” endpoints
* [ ] “Rainbow Six Siege” endpoints
* [ ] “Rocket League” endpoints
* [ ] “Valorant” endpoints
* [ ] “King of Glory” endpoints
* [ ] “StarCraft 2” endpoints
* [ ] “StarCraft Brood War” endpoints

### Examples

To search for a league by name:

```rust
use anyhow::Context;
use pandascore::{
    endpoint::{all::leagues::ListLeagues, CollectionOptions},
    Client,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = std::env::var("PANDASCORE_TOKEN").context("PANDASCORE_TOKEN missing")?;
    let search = std::env::args().nth(1).unwrap_or_else(|| "LCK".to_owned());

    let list_leagues = ListLeagues(CollectionOptions::new().search("name", search));

    let client = Client::new(reqwest::Client::new(), token)?;
    let response = client.execute(list_leagues).await?;
    println!("{:#?}", response);

    Ok(())
}

```

To get a player by ID or name:

```rust
use anyhow::Context;
use pandascore::{endpoint::all::players::GetPlayer, model::Identifier, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = std::env::var("PANDASCORE_TOKEN").context("PANDASCORE_TOKEN missing")?;
    let arg = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "faker".to_owned());

    let get_player = match arg.parse::<u64>() {
        Ok(id) => GetPlayer(Identifier::Id(id)),
        Err(_) => GetPlayer(Identifier::Slug(&arg)),
    };

    let client = Client::new(reqwest::Client::new(), token)?;
    let response = client.execute(get_player).await?;
    println!("{:#?}", response);

    Ok(())
}

```


 [__link0]: https://pandascore.co/
