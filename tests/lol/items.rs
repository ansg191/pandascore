use pandascore::{endpoint::lol::items::ListItems, Client};

use crate::common::{Expectation, MockClient};

#[tokio::test]
async fn test_list_items() {
    let client = MockClient::new(include_bytes!("../fixtures/lol/items_list.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/lol/items"));

    let client = Client::new(client, "").unwrap();

    let list_items = ListItems::default();
    let items = client.execute(list_items).await.unwrap();

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, 3029);
    assert_eq!(items[0].name, "Night Harvester");
    assert_eq!(items[0].gold_base, Some(815));
    assert_eq!(items[0].gold_total, Some(3200));
    assert_eq!(items[0].flat_hp_pool_mod, Some(300));
}
