use pandascore::{endpoint::rl::series::ListSeries, Client};

use crate::common::{Expectation, MockClient};

#[tokio::test]
async fn test_list_series() {
    let client = MockClient::new(include_bytes!("../fixtures/rl/series_list.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/rl/series/"));

    let client = Client::new(client, "").unwrap();

    let items = client.execute(ListSeries::default()).await.unwrap();

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, 6116);
    assert_eq!(items[0].league_id, 4834);
    assert_eq!(items[0].name, None);
    assert_eq!(items[0].full_name, "2023");
    assert_eq!(items[0].slug, "rl-rlcs-world-championship-2023");
    assert_eq!(items[0].year, 2023);
    assert_eq!(items[0].season, None);
}
