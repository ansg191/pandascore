use pandascore::{endpoint::rl::tournaments::ListTournaments, model::tournament::Tier, Client};

use crate::common::{Expectation, MockClient};

#[tokio::test]
async fn test_list_tournaments() {
    let client = MockClient::new(include_bytes!("../fixtures/rl/tournaments_list.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/rl/tournaments/"));

    let client = Client::new(client, "").unwrap();

    let items = client.execute(ListTournaments::default()).await.unwrap();

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, 11677);
    assert_eq!(items[0].league_id, 4823);
    assert_eq!(items[0].serie_id, 6522);
    assert_eq!(items[0].name, "Playoffs");
    assert_eq!(items[0].slug, "rl-gamers8-2023-playoffs");
    assert_eq!(items[0].tier, Some(Tier::B));
}
