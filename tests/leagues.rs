use pandascore::{
    endpoint::{
        all::leagues::{GetLeague, GetLeagueMatches, ListLeagueSeries, ListLeagues},
        CollectionOptions,
    },
    model::{EventStatus, Identifier},
    Client,
};

use crate::common::{Expectation, MockClient};

mod common;

#[tokio::test]
async fn test_list_leagues() {
    let client = MockClient::new(include_bytes!("./fixtures/list_leagues.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/leagues"));

    let client = Client::new(client, "").unwrap();

    let list_leagues = ListLeagues::default();
    let response = client.execute(list_leagues).await.unwrap();

    assert_eq!(response.len(), 1);
    assert_eq!(response[0].id, 5139);
    assert_eq!(response[0].name, "A1 Esport Valorant Cup");
}

#[tokio::test]
async fn test_list_leagues_options() {
    let client = MockClient::new(include_bytes!("./fixtures/list_leagues.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/leagues"))
        .expect(Expectation::Query("page[number]", "1"))
        .expect(Expectation::Query("page[size]", "10"))
        .expect(Expectation::Query("filter[name]", "A1 Esport Valorant Cup"))
        .expect(Expectation::Query("sort", "name"));

    let client = Client::new(client, "").unwrap();

    let list_leagues = ListLeagues(
        CollectionOptions::new()
            .page(1)
            .per_page(10)
            .filter("name", "A1 Esport Valorant Cup")
            .sort("name"),
    );
    let response = client.execute(list_leagues).await.unwrap();

    assert_eq!(response.len(), 1);
    assert_eq!(response[0].id, 5139);
    assert_eq!(response[0].name, "A1 Esport Valorant Cup");
}

#[tokio::test]
async fn test_get_league() {
    let client = MockClient::new(include_bytes!("./fixtures/get_league.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/leagues/4199"));

    let client = Client::new(client, "").unwrap();

    let get_league = GetLeague(Identifier::Id(4199));
    let response = client.execute(get_league).await.unwrap();

    assert_eq!(response.id, 4199);
    assert_eq!(response.name, "LLA");
    assert_eq!(response.slug, "league-of-legends-lla");
    assert_eq!(response.video_game.name, "LoL");
    assert_eq!(response.series.len(), 10);
    assert_eq!(response.series[0].id, 1708);
    assert_eq!(
        response.series[0].slug,
        "league-of-legends-lla-opening-2019"
    );
}

#[tokio::test]
async fn test_get_league_matches() {
    let client = MockClient::new(include_bytes!("./fixtures/get_league_matches.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/leagues/5139/matches/"));

    let client = Client::new(client, "").unwrap();

    let get_league_matches = GetLeagueMatches::builder().id(5139).build();
    let response = client.execute(get_league_matches).await.unwrap();

    assert_eq!(response.len(), 1);
    assert_eq!(response[0].id, 847936);
    assert_eq!(response[0].league_id, 5139);
    assert_eq!(response[0].name, "Grand final: TBD vs TBD");
    // assert_eq!(response[0].winner, Some(Winner::Team { id: None }));
}

#[tokio::test]
async fn test_get_league_upcoming_matches() {
    let client = MockClient::new(include_bytes!("./fixtures/get_league_matches.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/leagues/5139/matches/upcoming"));

    let client = Client::new(client, "").unwrap();

    let get_league_matches = GetLeagueMatches::builder()
        .id(5139)
        .status(EventStatus::Upcoming)
        .build();
    let response = client.execute(get_league_matches).await.unwrap();

    assert_eq!(response.len(), 1);
    assert_eq!(response[0].id, 847936);
    assert_eq!(response[0].league_id, 5139);
    assert_eq!(response[0].name, "Grand final: TBD vs TBD");
    // assert_eq!(response[0].winner, Some(Winner::Team { id: None }));
}

#[tokio::test]
async fn test_list_league_series() {
    let client = MockClient::new(include_bytes!("./fixtures/list_league_series.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/leagues/5139/series"));

    let client = Client::new(client, "").unwrap();

    let list_league_series = ListLeagueSeries::builder().id(5139).build();
    let response = client.execute(list_league_series).await.unwrap();

    assert_eq!(response.len(), 1);
    assert_eq!(response[0].id, 6714);
    assert_eq!(response[0].full_name, "Season 4 2023");
    assert_eq!(response[0].league_id, 5139);
}
