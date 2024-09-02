use pandascore::{endpoint::rl::matches::ListMatches, Client};

use crate::common::{Expectation, MockClient};

#[tokio::test]
async fn test_list_matches() {
    let client = MockClient::new(include_bytes!("../fixtures/rl/matches_list.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/rl/matches/"));

    let client = Client::new(client, "").unwrap();

    let items = client.execute(ListMatches::default()).await.unwrap();

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, 590910);
    assert_eq!(items[0].league_id, 4245);
    assert_eq!(items[0].serie_id, 3568);
    assert_eq!(items[0].tournament_id, 5951);
    assert_eq!(items[0].name, "G2 vs NRG");
}
