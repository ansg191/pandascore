use pandascore::{endpoint::lol::leagues::ListLeagues, Client};

use crate::common::{Expectation, MockClient};

#[tokio::test]
async fn test_list_leagues() {
    let client = MockClient::new(include_bytes!("../fixtures/lol/leagues_list.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/lol/leagues"));

    let client = Client::new(client, "").unwrap();

    let items = client.execute(ListLeagues::default()).await.unwrap();

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, 4864);
    assert_eq!(items[0].name, "Asia Star Challengers Invitational");
    assert_eq!(items[0].slug, "league-of-legends-asia-star-challengers-invitational");
    assert_eq!(items[0].series.len(), 1);
    assert_eq!(items[0].video_game.name, "LoL");
    assert_eq!(items[0].video_game.slug, "league-of-legends");
}
