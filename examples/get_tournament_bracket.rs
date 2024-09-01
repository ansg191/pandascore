use anyhow::Context;
use pandascore::{endpoint::all::tournament::GetTournamentBracket, model::Identifier, Client};
use petgraph::dot::Dot;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = std::env::var("PANDASCORE_TOKEN").context("PANDASCORE_TOKEN missing")?;
    let arg = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "14032".to_owned());

    let get_league = match arg.parse::<u64>() {
        Ok(id) => GetTournamentBracket(Identifier::Id(id)),
        Err(_) => GetTournamentBracket(Identifier::Slug(&arg)),
    };

    let client = Client::new(reqwest::Client::new(), token)?;
    let response = client.execute(get_league).await?;

    println!("{}", Dot::new(response.as_ref()));

    Ok(())
}
