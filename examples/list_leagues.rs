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
    println!("{response:#?}");

    Ok(())
}
