use anyhow::Context;
use pandascore::{endpoint::all::players::GetPlayer, model::Identifier, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = std::env::var("PANDASCORE_TOKEN").context("PANDASCORE_TOKEN missing")?;
    let arg = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "faker".to_owned());

    let get_player = GetPlayer(
        arg.parse::<u64>()
            .map_or_else(|_| Identifier::Slug(&arg), Identifier::Id),
    );

    let client = Client::new(reqwest::Client::new(), token)?;
    let response = client.execute(get_player).await?;
    println!("{response:#?}");

    Ok(())
}
