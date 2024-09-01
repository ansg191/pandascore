use pandascore::{endpoint::lol::players::ListPlayers, Client};

use crate::common::{Expectation, MockClient};

#[tokio::test]
async fn test_list_players() {
    let client = MockClient::new(include_bytes!("../fixtures/lol/players_list.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/lol/players"));

    let client = Client::new(client, "").unwrap();

    let items = client.execute(ListPlayers::default()).await.unwrap();

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, 51237);
    assert_eq!(items[0].name, "Traffy");
    assert_eq!(items[0].slug, Some("traffy".into()));
    assert_eq!(items[0].first_name, Some("Adam".into()));
    assert_eq!(items[0].last_name, Some("Malek".into()));
    assert_eq!(items[0].role, Some("sup".into()));
    assert_eq!(items[0].nationality, Some("PL".into()));
}
