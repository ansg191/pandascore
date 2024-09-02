use pandascore::{
    endpoint::all::series::{GetSeries, ListSeries},
    Client,
};

use crate::common::{Expectation, MockClient};

mod common;

#[tokio::test]
async fn test_list_series() {
    let client = MockClient::new(include_bytes!("./fixtures/series_list.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/series/"));

    let client = Client::new(client, "").unwrap();

    let response = client.execute(ListSeries::default()).await.unwrap();

    assert_eq!(response.len(), 1);
    assert_eq!(response[0].id, 6714);
    assert_eq!(response[0].full_name, "Season 4 2023");
    assert_eq!(response[0].league_id, 5139);
    assert_eq!(response[0].name, None);
    assert_eq!(response[0].season, Some("4".into()));
    assert_eq!(response[0].slug, "valorant-a1-esport-league-4-2023");
    assert_eq!(response[0].year, 2023);
}

#[tokio::test]
async fn test_get_series() {
    let client = MockClient::new(include_bytes!("./fixtures/series_get.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/series/1626"));

    let client = Client::new(client, "").unwrap();

    let response = client.execute(GetSeries::from(1626)).await.unwrap();

    assert_eq!(response.id, 1626);
    assert_eq!(response.full_name, "Season 6 2018");
    assert_eq!(response.league_id, 4165);
    assert_eq!(response.name, None);
    assert_eq!(response.season, Some("6".into()));
    assert_eq!(response.slug, "cs-go-starseries-i-league-6-2018");
    assert_eq!(response.year, 2018);
}
