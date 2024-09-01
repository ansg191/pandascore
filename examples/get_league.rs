use anyhow::Context;
use pandascore::{endpoint::all::leagues::GetLeague, model::Identifier, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = std::env::var("PANDASCORE_TOKEN").context("PANDASCORE_TOKEN missing")?;
    let arg = std::env::args().nth(1).unwrap_or_else(|| "293".to_owned());

    let get_league = match arg.parse::<u64>() {
        Ok(id) => GetLeague(Identifier::Id(id)),
        Err(_) => GetLeague(Identifier::Slug(&arg)),
    };

    let client = Client::new(reqwest::Client::new(), token)?;
    let response = client.execute(get_league).await?;
    println!("{:#?}", response);

    Ok(())
}
