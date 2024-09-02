use pandascore::{
    endpoint::all::tournament::{GetTournamentRosters, GetTournamentStandings},
    model::tournament::{TournamentRosters, TournamentStanding},
    Client,
};

use crate::common::{Expectation, MockClient};

mod common;

#[tokio::test]
async fn test_get_tournament_rosters() {
    let client = MockClient::new(include_bytes!("./fixtures/tournaments_rosters_get.json"))
        .expect(Expectation::Method(reqwest::Method::GET))
        .expect(Expectation::Path("/tournaments/1/rosters"));

    let client = Client::new(client, "").unwrap();

    let response = client.execute(GetTournamentRosters::from(1)).await.unwrap();

    assert!(matches!(response, TournamentRosters::Team(_)));
    let TournamentRosters::Team(teams) = response else {
        unreachable!();
    };
    assert_eq!(teams.len(), 9);
    assert_eq!(teams[0].id, 126_059);
    assert_eq!(teams[0].name, "Top Esports");
}

#[tokio::test]
async fn test_get_tournament_standings_bracket() {
    let client = MockClient::new(include_bytes!(
        "./fixtures/tournaments_standings_bracket_get.json"
    ))
    .expect(Expectation::Method(reqwest::Method::GET))
    .expect(Expectation::Path("/tournaments/1/standings"));

    let client = Client::new(client, "").unwrap();

    let response = client
        .execute(GetTournamentStandings::builder().id(1).build())
        .await
        .unwrap();

    assert_eq!(response.len(), 1);
    assert!(matches!(response[0], TournamentStanding::Bracket(_)));
    let TournamentStanding::Bracket(bracket) = &response[0] else {
        unreachable!()
    };
    assert_eq!(bracket.rank, 1);
    assert_eq!(bracket.team.name, "JD Gaming");
    assert_eq!(bracket.last_match.id, 53995);
}

#[tokio::test]
async fn test_get_tournament_standings_group() {
    let client = MockClient::new(include_bytes!(
        "./fixtures/tournaments_standings_group_get.json"
    ))
    .expect(Expectation::Method(reqwest::Method::GET))
    .expect(Expectation::Path("/tournaments/1/standings"));

    let client = Client::new(client, "").unwrap();

    let response = client
        .execute(GetTournamentStandings::builder().id(1).build())
        .await
        .unwrap();

    assert_eq!(response.len(), 1);
    assert!(matches!(response[0], TournamentStanding::Group(_)));
    let TournamentStanding::Group(group) = &response[0] else {
        unreachable!()
    };
    assert_eq!(group.losses, 5);
    assert_eq!(group.wins, 13);
    assert_eq!(group.rank, 1);
    assert_eq!(group.total, 18);
    assert_eq!(group.team.name, "Fnatic");
}
