use std::future::Future;

use reqwest::{Request, Response};
use url::Url;

use crate::{
    endpoint::{
        sealed::Sealed, CollectionOptions, EndpointError, ListResponse, PaginatedEndpoint, BASE_URL,
    },
    model::{league::League, Identifier},
};

crate::endpoint::list_endpoint!(ListPlayers("/players") => crate::model::player::Player);
crate::endpoint::get_endpoint!(GetPlayer("/players") => crate::model::player::Player);

#[derive(Debug, Clone, PartialEq, Eq, bon::Builder)]
pub struct ListPlayerLeagues<'a> {
    #[builder(into)]
    id: Identifier<'a>,
    #[builder(default)]
    options: CollectionOptions,
}

impl Sealed for ListPlayerLeagues<'_> {
    type Response = ListResponse<League>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/players/{}/leagues", BASE_URL, self.id))?;
        self.options.add_params(&mut url);
        Ok(Request::new(reqwest::Method::GET, url))
    }

    fn from_response(
        response: Response,
    ) -> impl Future<Output = Result<Self::Response, EndpointError>> + Send {
        ListResponse::from_response(response)
    }
}

impl PaginatedEndpoint for ListPlayerLeagues<'_> {
    type Item = League;

    fn with_options(self, options: CollectionOptions) -> Self {
        Self { options, ..self }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, bon::Builder)]
pub struct ListPlayerSeries<'a> {
    #[builder(into)]
    id: Identifier<'a>,
    #[builder(default)]
    options: CollectionOptions,
}

impl Sealed for ListPlayerSeries<'_> {
    type Response = ListResponse<League>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/players/{}/series", BASE_URL, self.id))?;
        self.options.add_params(&mut url);
        Ok(Request::new(reqwest::Method::GET, url))
    }

    fn from_response(
        response: Response,
    ) -> impl Future<Output = Result<Self::Response, EndpointError>> + Send {
        ListResponse::from_response(response)
    }
}

impl PaginatedEndpoint for ListPlayerSeries<'_> {
    type Item = League;

    fn with_options(self, options: CollectionOptions) -> Self {
        Self { options, ..self }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, bon::Builder)]
pub struct ListPlayerTournaments<'a> {
    #[builder(into)]
    id: Identifier<'a>,
    #[builder(default)]
    options: CollectionOptions,
}

impl Sealed for ListPlayerTournaments<'_> {
    type Response = ListResponse<League>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/players/{}/tournaments", BASE_URL, self.id))?;
        self.options.add_params(&mut url);
        Ok(Request::new(reqwest::Method::GET, url))
    }

    fn from_response(
        response: Response,
    ) -> impl Future<Output = Result<Self::Response, EndpointError>> + Send {
        ListResponse::from_response(response)
    }
}

impl PaginatedEndpoint for ListPlayerTournaments<'_> {
    type Item = League;

    fn with_options(self, options: CollectionOptions) -> Self {
        Self { options, ..self }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, bon::Builder)]
pub struct ListPlayerMatches<'a> {
    #[builder(into)]
    id: Identifier<'a>,
    #[builder(default)]
    options: CollectionOptions,
}

impl Sealed for ListPlayerMatches<'_> {
    type Response = ListResponse<League>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/players/{}/matches", BASE_URL, self.id))?;
        self.options.add_params(&mut url);
        Ok(Request::new(reqwest::Method::GET, url))
    }

    fn from_response(
        response: Response,
    ) -> impl Future<Output = Result<Self::Response, EndpointError>> + Send {
        ListResponse::from_response(response)
    }
}

impl PaginatedEndpoint for ListPlayerMatches<'_> {
    type Item = League;

    fn with_options(self, options: CollectionOptions) -> Self {
        Self { options, ..self }
    }
}
