use pandascore::{endpoint::lol::series::ListSeries, Client};

use crate::common::{Expectation, MockClient};

#[tokio::test]
async fn test_list_series() {
    let client = MockClient::new(include_bytes!("../fixtures/lol/series_list.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/lol/series/"));

    let client = Client::new(client, "").unwrap();

    let items = client.execute(ListSeries::default()).await.unwrap();

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, 6059);
    assert_eq!(items[0].league_id, 4969);
    assert_eq!(items[0].name, None);
    assert_eq!(items[0].full_name, "Summer 2023");
    assert_eq!(
        items[0].slug,
        "league-of-legends-lvp-sl-2nd-division-summer-2023"
    );
    assert_eq!(items[0].year, 2023);
    assert_eq!(items[0].season, Some("Summer".into()));
}
