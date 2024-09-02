use pandascore::{endpoint::rl::players::ListPlayers, Client};

use crate::common::{Expectation, MockClient};

#[tokio::test]
async fn test_list_players() {
    let client = MockClient::new(include_bytes!("../fixtures/rl/players_list.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/rl/players"));

    let client = Client::new(client, "").unwrap();

    let items = client.execute(ListPlayers::default()).await.unwrap();

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, 51077);
    assert_eq!(items[0].name, "dralii");
    assert_eq!(items[0].slug, Some("dralii".into()));
    assert_eq!(items[0].first_name, None);
    assert_eq!(items[0].last_name, None);
    assert_eq!(items[0].role, None);
    assert_eq!(items[0].nationality, Some("MA".into()));
}
