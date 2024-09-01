use pandascore::{endpoint::lol::tournaments::ListTournaments, model::tournament::Tier, Client};

use crate::common::{Expectation, MockClient};

#[tokio::test]
async fn test_list_tournaments() {
    let client = MockClient::new(include_bytes!("../fixtures/lol/tournaments_list.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/lol/tournaments/"));

    let client = Client::new(client, "").unwrap();

    let items = client.execute(ListTournaments::default()).await.unwrap();

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, 11871);
    assert_eq!(items[0].league_id, 297);
    assert_eq!(items[0].serie_id, 6560);
    assert_eq!(items[0].name, "Play-In Elimination");
    assert_eq!(
        items[0].slug,
        "league-of-legends-world-championship-2023-play-in-elimination"
    );
    assert_eq!(items[0].tier, Some(Tier::S));
}
