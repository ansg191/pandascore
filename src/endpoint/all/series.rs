use derive_builder::Builder;
use reqwest::{Request, Response};
use url::Url;

use crate::{
    endpoint::{sealed::Sealed, CollectionOptions, EndpointError, ListResponse, BASE_URL},
    model::{matches::Match, series::Series, tournament::Tournament, EventStatus, Identifier},
};

crate::endpoint::multi_list_endpoint!(ListSeries("/series") => Series);
crate::endpoint::get_endpoint!(GetSeries("/series") => Series);

#[derive(Debug, Clone, PartialEq, Eq, Builder)]
#[builder(build_fn(error = "crate::endpoint::BuilderError"))]
pub struct ListSeriesMatches<'a> {
    #[builder(setter(into))]
    id: Identifier<'a>,
    #[builder(default)]
    status: Option<EventStatus>,
    #[builder(default)]
    options: CollectionOptions,
}

impl ListSeriesMatches<'_> {
    pub fn builder() -> ListSeriesMatchesBuilder<'static> {
        ListSeriesMatchesBuilder::create_empty()
    }
}

impl Sealed for ListSeriesMatches<'_> {
    type Response = ListResponse<Match>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/series/{}/matches/", BASE_URL, self.id,))?;
        self.options.add_params(&mut url);
        if let Some(status) = self.status {
            url = url.join(status.as_str())?;
        }
        Ok(Request::new(reqwest::Method::GET, url))
    }

    async fn from_response(response: Response) -> Result<Self::Response, EndpointError> {
        ListResponse::from_response(response).await
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Builder)]
#[builder(build_fn(error = "crate::endpoint::BuilderError"))]
pub struct ListSeriesTournaments<'a> {
    #[builder(setter(into))]
    id: Identifier<'a>,
    #[builder(default)]
    options: CollectionOptions,
}

impl ListSeriesTournaments<'_> {
    pub fn builder() -> ListSeriesTournamentsBuilder<'static> {
        ListSeriesTournamentsBuilder::create_empty()
    }
}

impl Sealed for ListSeriesTournaments<'_> {
    type Response = ListResponse<Tournament>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/series/{}/tournaments", BASE_URL, self.id,))?;
        self.options.add_params(&mut url);
        Ok(Request::new(reqwest::Method::GET, url))
    }

    async fn from_response(response: Response) -> Result<Self::Response, EndpointError> {
        ListResponse::from_response(response).await
    }
}