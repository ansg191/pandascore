use pandascore::{
    endpoint::all::players::{GetPlayer, ListPlayers},
    model::Identifier,
    Client,
};
use time::{Date, Month};

use crate::common::{Expectation, MockClient};

mod common;

#[tokio::test]
async fn test_list_players() {
    let client = MockClient::new(include_bytes!("./fixtures/list_players.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/players"));

    let client = Client::new(client, "").unwrap();

    let list_players = ListPlayers::default();
    let response = client.execute(list_players).await.unwrap();

    assert_eq!(response.len(), 1);
    assert_eq!(response[0].id, 585);
    assert_eq!(response[0].name, "Faker");
    assert_eq!(response[0].slug, Some("faker".into()));
    assert_eq!(response[0].first_name, Some("Lee".into()));
    assert_eq!(response[0].last_name, Some("Sang-hyeok".into()));
    assert_eq!(response[0].role, Some("mid".into()));
    assert_eq!(response[0].nationality, Some("KR".into()));
}

#[tokio::test]
async fn test_get_player() {
    let client = MockClient::new(include_bytes!("./fixtures/get_player.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/players/faker"));

    let client = Client::new(client, "").unwrap();

    let get_player = GetPlayer(Identifier::Slug("faker"));
    let response = client.execute(get_player).await.unwrap();

    assert_eq!(response.id, 585);
    assert_eq!(response.name, "Faker");
    assert_eq!(response.slug, Some("faker".into()));
    assert_eq!(response.first_name, Some("Lee".into()));
    assert_eq!(response.last_name, Some("Sang-hyeok".into()));
    assert_eq!(response.role, Some("mid".into()));
    assert_eq!(response.nationality, Some("KR".into()));
    assert_eq!(
        response.birthday,
        Some(Date::from_calendar_date(1996, Month::May, 7).unwrap())
    );
}
