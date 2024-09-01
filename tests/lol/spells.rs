use pandascore::{endpoint::lol::spells::ListSpells, Client};

use crate::common::{Expectation, MockClient};

#[tokio::test]
async fn test_list_spells() {
    let client = MockClient::new(include_bytes!("../fixtures/lol/spells_list.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/lol/spells"));

    let client = Client::new(client, "").unwrap();

    let items = client.execute(ListSpells::default()).await.unwrap();

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, 122);
    assert_eq!(items[0].name, "Placeholder and Attack-Smite");
    assert_eq!(
        items[0].image_url,
        "https://cdn.pandascore.co/images/lol/spell/image/3a9e0f3dd1b09b665189bcd378a639fa.png"
    );
}
