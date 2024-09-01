use pandascore::{
    endpoint::lol::spells::{GetSpell, ListSpells},
    Client,
};

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

#[tokio::test]
async fn test_get_spell() {
    let client = MockClient::new(include_bytes!("../fixtures/lol/spells_get.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/lol/spells/39"));

    let client = Client::new(client, "").unwrap();

    let spell = client.execute(GetSpell(39)).await.unwrap();

    assert_eq!(spell.id, 39);
    assert_eq!(spell.name, "Teleport");
    assert_eq!(
        spell.image_url,
        "https://cdn.pandascore.co/images/lol/spell/image/76f3d1cbc7111379f6efe2d1098b04c4.png"
    );
}
