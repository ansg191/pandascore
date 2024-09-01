use pandascore::{
    endpoint::{
        all::teams::{GetTeam, ListTeams},
        CollectionOptions,
    },
    model::Identifier,
    Client,
};

use crate::common::{Expectation, MockClient};

mod common;

#[tokio::test]
async fn test_list_teams() {
    let client = MockClient::new(include_bytes!("./fixtures/list_teams.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/teams"))
        .expect(Expectation::Query("filter[name]", "T1"))
        .expect(Expectation::Query("filter[location]", "KR"));

    let client = Client::new(client, "").unwrap();

    let list_teams = ListTeams(
        CollectionOptions::new()
            .filter("name", "T1")
            .filter("location", "KR"),
    );
    let response = client.execute(list_teams).await.unwrap();

    assert_eq!(response.len(), 6);
    assert_eq!(response[4].acronym, Some("T1".into()));
    assert_eq!(response[4].id, 126061);
    assert_eq!(response[4].players.len(), 6);
    assert_eq!(response[4].players[0].name, "Faker");
}

#[tokio::test]
async fn test_get_team() {
    let client = MockClient::new(include_bytes!("./fixtures/get_team.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/teams/1804"));

    let client = Client::new(client, "").unwrap();

    let get_team = GetTeam(Identifier::Id(1804));
    let response = client.execute(get_team).await.unwrap();

    assert_eq!(response.id, 1804);
    assert_eq!(response.acronym, Some("VGJ.T".into()));
    assert_eq!(response.players.len(), 0);
    assert_eq!(response.name, "VGJ.Thunder");
    assert_eq!(response.location, Some("CN".into()));
}
