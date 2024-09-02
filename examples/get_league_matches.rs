use anyhow::Context;
use pandascore::{
    endpoint::all::leagues::GetLeagueMatches,
    model::{EventStatus, Identifier},
    Client,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = std::env::var("PANDASCORE_TOKEN").context("PANDASCORE_TOKEN missing")?;
    let arg = std::env::args().nth(1).unwrap_or_else(|| "293".to_owned());
    let status = std::env::args().nth(2);

    let id = arg
        .parse::<u64>()
        .map_or_else(|_| Identifier::Slug(&arg), Identifier::Id);
    let status = match status.as_deref() {
        None => None,
        Some("past") => Some(EventStatus::Past),
        Some("running") => Some(EventStatus::Running),
        Some("upcoming") => Some(EventStatus::Upcoming),
        Some(status) => return Err(anyhow::anyhow!("Invalid status: {}", status)),
    };
    let get_league_matches = GetLeagueMatches::builder()
        .id(id)
        .maybe_status(status)
        .build();

    let client = Client::new(reqwest::Client::new(), token)?;
    let response = client.execute(get_league_matches).await?;
    println!("{response:#?}");

    Ok(())
}
