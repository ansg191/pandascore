use anyhow::Context;
use pandascore::{endpoint::all::leagues::GetLeague, model::Identifier, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = std::env::var("PANDASCORE_TOKEN").context("PANDASCORE_TOKEN missing")?;
    let arg = std::env::args().nth(1).unwrap_or_else(|| "293".to_owned());

    let get_league = arg.parse::<u64>().map_or_else(
        |_| GetLeague(Identifier::Slug(&arg)),
        |id| GetLeague(Identifier::Id(id)),
    );

    let client = Client::new(reqwest::Client::new(), token)?;
    let response = client.execute(get_league).await?;
    println!("{response:#?}");

    Ok(())
}
