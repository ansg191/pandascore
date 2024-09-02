use pandascore::{endpoint::rl::teams::ListTeams, Client};

use crate::common::{Expectation, MockClient};

#[tokio::test]
async fn test_list_leagues() {
    let client = MockClient::new(include_bytes!("../fixtures/rl/teams_list.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/rl/teams"));

    let client = Client::new(client, "").unwrap();

    let items = client.execute(ListTeams::default()).await.unwrap();

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, 133806);
    assert_eq!(items[0].name, "Top Cougars");
    assert_eq!(items[0].slug, Some("top-cougars".into()));
    assert_eq!(items[0].acronym, Some("TP".into()));
    assert_eq!(items[0].location, Some("".into()));
}
