use pandascore::{endpoint::rl::leagues::ListLeagues, Client};

use crate::common::{Expectation, MockClient};

#[tokio::test]
async fn test_list_leagues() {
    let client = MockClient::new(include_bytes!("../fixtures/rl/leagues_list.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/rl/leagues"));

    let client = Client::new(client, "").unwrap();

    let items = client.execute(ListLeagues::default()).await.unwrap();

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, 4871);
    assert_eq!(items[0].name, "rle.gg Roster Royale");
    assert_eq!(items[0].slug, "rl-rle-gg-roster-royale");
    assert_eq!(items[0].series.len(), 1);
    assert_eq!(items[0].video_game.name, "Rocket League");
    assert_eq!(items[0].video_game.slug, "rl");
}
