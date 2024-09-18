use common::{Expectation, MockClient};
use pandascore::{
    endpoint::all::matches::GetMatch,
    model::{
        matches::{Match, MatchStatus, MatchType},
        Identifier, Winner,
    },
    Client,
};

mod common;

#[test]
fn test_deserialize_match() {
    let json = include_str!("./fixtures/match_de.json");
    let m: Match = serde_json::from_str(json).unwrap();

    assert_eq!(m.id, 1000450);
    assert_eq!(m.name, "Grand final: GEN vs HLE");
    assert_eq!(m.match_type, MatchType::BestOf);
    assert_eq!(m.number_of_games, 5);
    assert_eq!(m.status, MatchStatus::Finished);
    assert!(matches!(m.winner, Some(Winner::Team { .. })))
}

#[tokio::test]
async fn test_get_match() {
    let client = MockClient::new(include_bytes!("./fixtures/match_get.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/matches/1000450"));

    let client = Client::new(client, "").unwrap();

    let get_match = GetMatch(Identifier::Id(1000450));
    let response = client.execute(get_match).await.unwrap();

    assert_eq!(response.id, 1000450);
    assert_eq!(response.name, "Grand final: GEN vs HLE");
    assert_eq!(response.match_type, MatchType::BestOf);
    assert_eq!(response.number_of_games, 5);
    assert_eq!(response.status, MatchStatus::Finished);
    assert!(matches!(response.winner, Some(Winner::Team { .. })))
}
