use std::future::Future;

use reqwest::{Request, Response};
use url::Url;

use crate::{
    endpoint::{sealed::Sealed, CollectionOptions, EndpointError, ListResponse, BASE_URL},
    model::{
        bracket::{TournamentBracket, TournamentBracketMatch},
        matches::Match,
        team::Team,
        tournament::{TournamentRosters, TournamentStanding},
        Identifier,
    },
};

crate::endpoint::multi_list_endpoint!(ListTournaments("/tournaments") => crate::model::tournament::Tournament);
crate::endpoint::get_endpoint!(GetTournament("/tournaments") => crate::model::tournament::Tournament);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GetTournamentBracket<'a>(pub Identifier<'a>);

//TODO: add pagination
impl Sealed for GetTournamentBracket<'_> {
    type Response = TournamentBracket;

    fn to_request(self) -> Result<Request, EndpointError> {
        let url = Url::parse(&format!("{}/tournaments/{}/brackets", BASE_URL, self.0))?;
        Ok(Request::new(reqwest::Method::GET, url))
    }

    async fn from_response(response: Response) -> Result<Self::Response, EndpointError> {
        let bracket: Vec<TournamentBracketMatch> =
            crate::endpoint::deserialize(response.error_for_status()?).await?;
        Ok(TournamentBracket::new(bracket))
    }
}

impl<'a, T> From<T> for GetTournamentBracket<'a>
where
    T: Into<Identifier<'a>>,
{
    fn from(id: T) -> Self {
        Self(id.into())
    }
}

#[bon::builder]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListTournamentMatches<'a> {
    #[builder(into)]
    pub id: Identifier<'a>,
    #[builder(default)]
    pub options: CollectionOptions,
}

impl Sealed for ListTournamentMatches<'_> {
    type Response = ListResponse<Match>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/tournaments/{}/matches", BASE_URL, self.id))?;
        self.options.add_params(&mut url);
        Ok(Request::new(reqwest::Method::GET, url))
    }

    fn from_response(
        response: Response,
    ) -> impl Future<Output = Result<Self::Response, EndpointError>> + Send {
        ListResponse::from_response(response)
    }
}

#[bon::builder]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListTournamentTeams<'a> {
    #[builder(into)]
    pub id: Identifier<'a>,
    #[builder(default)]
    pub options: CollectionOptions,
}

impl Sealed for ListTournamentTeams<'_> {
    type Response = ListResponse<Team>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/tournaments/{}/teams", BASE_URL, self.id))?;
        self.options.add_params(&mut url);
        Ok(Request::new(reqwest::Method::GET, url))
    }

    fn from_response(
        response: Response,
    ) -> impl Future<Output = Result<Self::Response, EndpointError>> + Send {
        ListResponse::from_response(response)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GetTournamentRosters<'a>(pub Identifier<'a>);

impl Sealed for GetTournamentRosters<'_> {
    type Response = TournamentRosters;

    fn to_request(self) -> Result<Request, EndpointError> {
        let url = Url::parse(&format!("{}/tournaments/{}/rosters", BASE_URL, self.0))?;
        Ok(Request::new(reqwest::Method::GET, url))
    }

    async fn from_response(response: Response) -> Result<Self::Response, EndpointError> {
        crate::endpoint::deserialize(response.error_for_status()?).await
    }
}

impl<'a, T> From<T> for GetTournamentRosters<'a>
where
    T: Into<Identifier<'a>>,
{
    fn from(id: T) -> Self {
        Self(id.into())
    }
}

#[bon::builder]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GetTournamentStandings<'a> {
    #[builder(into)]
    pub id: Identifier<'a>,
    #[builder(default)]
    pub options: CollectionOptions,
}

impl Sealed for GetTournamentStandings<'_> {
    type Response = ListResponse<TournamentStanding>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/tournaments/{}/standings", BASE_URL, self.id))?;
        self.options.add_params(&mut url);
        Ok(Request::new(reqwest::Method::GET, url))
    }

    async fn from_response(response: Response) -> Result<Self::Response, EndpointError> {
        ListResponse::from_response(response).await
    }
}
