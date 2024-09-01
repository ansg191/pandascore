use pandascore::{endpoint::lol::matches::ListMatches, Client};

use crate::common::{Expectation, MockClient};

#[tokio::test]
async fn test_list_matches() {
    let client = MockClient::new(include_bytes!("../fixtures/lol/matches_list.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/lol/matches/"));

    let client = Client::new(client, "").unwrap();

    let items = client.execute(ListMatches::default()).await.unwrap();

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, 841976);
    assert_eq!(items[0].league_id, 297);
    assert_eq!(items[0].serie_id, 6560);
    assert_eq!(items[0].tournament_id, 11808);
    assert_eq!(items[0].name, "Round 5: TBD vs TBD");
}
