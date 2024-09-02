use pandascore::{endpoint::lol::teams::ListTeams, Client};

use crate::common::{Expectation, MockClient};

#[tokio::test]
async fn test_list_leagues() {
    let client = MockClient::new(include_bytes!("../fixtures/lol/teams_list.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/lol/teams"));

    let client = Client::new(client, "").unwrap();

    let items = client.execute(ListTeams::default()).await.unwrap();

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, 133_844);
    assert_eq!(items[0].name, "Duma Easta");
    assert_eq!(items[0].slug, Some("duma-easta".into()));
    assert_eq!(items[0].acronym, Some("DE".into()));
    assert_eq!(items[0].location, Some("PL".into()));
}
