use std::future::Future;

use reqwest::{Request, Response};
use url::Url;

use crate::{
    endpoint::{
        sealed::Sealed, CollectionOptions, EndpointError, ListResponse, PaginatedEndpoint, BASE_URL,
    },
    model::{league::League, matches::Match, series::Series, tournament::Tournament, Identifier},
};

crate::endpoint::list_endpoint!(ListTeams("/teams") => crate::model::team::Team);
crate::endpoint::get_endpoint!(GetTeam("/teams") => crate::model::team::Team);

#[derive(Debug, Clone, PartialEq, Eq, bon::Builder)]
pub struct ListTeamLeagues<'a> {
    #[builder(into)]
    id: Identifier<'a>,
    #[builder(default)]
    options: CollectionOptions,
}

impl Sealed for ListTeamLeagues<'_> {
    type Response = ListResponse<League>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/teams/{}/leagues", BASE_URL, self.id))?;
        self.options.add_params(&mut url);
        Ok(Request::new(reqwest::Method::GET, url))
    }

    fn from_response(
        response: Response,
    ) -> impl Future<Output = Result<Self::Response, EndpointError>> + Send {
        ListResponse::from_response(response)
    }
}

impl PaginatedEndpoint for ListTeamLeagues<'_> {
    type Item = League;

    fn with_options(self, options: CollectionOptions) -> Self {
        Self { options, ..self }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, bon::Builder)]
pub struct ListTeamSeries<'a> {
    #[builder(into)]
    id: Identifier<'a>,
    #[builder(default)]
    options: CollectionOptions,
}

impl Sealed for ListTeamSeries<'_> {
    type Response = ListResponse<Series>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/teams/{}/series", BASE_URL, self.id))?;
        self.options.add_params(&mut url);
        Ok(Request::new(reqwest::Method::GET, url))
    }

    fn from_response(
        response: Response,
    ) -> impl Future<Output = Result<Self::Response, EndpointError>> + Send {
        ListResponse::from_response(response)
    }
}

impl PaginatedEndpoint for ListTeamSeries<'_> {
    type Item = Series;

    fn with_options(self, options: CollectionOptions) -> Self {
        Self { options, ..self }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, bon::Builder)]
pub struct ListTeamTournaments<'a> {
    #[builder(into)]
    id: Identifier<'a>,
    #[builder(default)]
    options: CollectionOptions,
}

impl Sealed for ListTeamTournaments<'_> {
    type Response = ListResponse<Tournament>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/teams/{}/tournaments", BASE_URL, self.id))?;
        self.options.add_params(&mut url);
        Ok(Request::new(reqwest::Method::GET, url))
    }

    fn from_response(
        response: Response,
    ) -> impl Future<Output = Result<Self::Response, EndpointError>> + Send {
        ListResponse::from_response(response)
    }
}

impl PaginatedEndpoint for ListTeamTournaments<'_> {
    type Item = Tournament;

    fn with_options(self, options: CollectionOptions) -> Self {
        Self { options, ..self }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, bon::Builder)]
pub struct ListTeamMatches<'a> {
    #[builder(into)]
    id: Identifier<'a>,
    #[builder(default)]
    options: CollectionOptions,
}

impl Sealed for ListTeamMatches<'_> {
    type Response = ListResponse<Match>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/teams/{}/matches", BASE_URL, self.id))?;
        self.options.add_params(&mut url);
        Ok(Request::new(reqwest::Method::GET, url))
    }

    fn from_response(
        response: Response,
    ) -> impl Future<Output = Result<Self::Response, EndpointError>> + Send {
        ListResponse::from_response(response)
    }
}

impl PaginatedEndpoint for ListTeamMatches<'_> {
    type Item = Match;

    fn with_options(self, options: CollectionOptions) -> Self {
        Self { options, ..self }
    }
}
