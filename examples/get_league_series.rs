use anyhow::Context;
use pandascore::{endpoint::all::leagues::ListLeagueSeries, model::Identifier, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = std::env::var("PANDASCORE_TOKEN").context("PANDASCORE_TOKEN missing")?;
    let arg = std::env::args().nth(1).unwrap_or_else(|| "293".to_owned());

    let id = match arg.parse::<u64>() {
        Ok(id) => Identifier::Id(id),
        Err(_) => Identifier::Slug(&arg),
    };
    let get_league_series = ListLeagueSeries::builder().id(id).build()?;

    let client = Client::new(reqwest::Client::new(), token)?;
    let response = client.execute(get_league_series).await?;
    println!("{:#?}", response);

    Ok(())
}
