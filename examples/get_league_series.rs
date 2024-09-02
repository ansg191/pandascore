use anyhow::Context;
use pandascore::{endpoint::all::leagues::ListLeagueSeries, model::Identifier, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = std::env::var("PANDASCORE_TOKEN").context("PANDASCORE_TOKEN missing")?;
    let arg = std::env::args().nth(1).unwrap_or_else(|| "293".to_owned());

    let id = arg
        .parse::<u64>()
        .map_or_else(|_| Identifier::Slug(&arg), Identifier::Id);
    let get_league_series = ListLeagueSeries::builder().id(id).build();

    let client = Client::new(reqwest::Client::new(), token)?;
    let response = client.execute(get_league_series).await?;
    println!("{response:#?}");

    Ok(())
}
