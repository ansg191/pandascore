use reqwest::{Request, Response};
use url::Url;

use crate::{
    endpoint::{
        sealed::Sealed, CollectionOptions, EndpointError, ListResponse, PaginatedEndpoint, BASE_URL,
    },
    model::{matches::Match, series::Series, tournament::Tournament, EventStatus, Identifier},
};

crate::endpoint::multi_list_endpoint!(ListSeries("/series") => Series);
crate::endpoint::get_endpoint!(GetSeries("/series") => Series);

#[derive(Debug, Clone, PartialEq, Eq, bon::Builder)]
pub struct ListSeriesMatches<'a> {
    #[builder(into)]
    id: Identifier<'a>,
    status: Option<EventStatus>,
    #[builder(default)]
    options: CollectionOptions,
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

#[derive(Debug, Clone, PartialEq, Eq, bon::Builder)]
pub struct ListSeriesTournaments<'a> {
    #[builder(into)]
    id: Identifier<'a>,
    #[builder(default)]
    options: CollectionOptions,
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

impl PaginatedEndpoint for ListSeriesTournaments<'_> {
    type Item = Tournament;

    fn with_options(self, options: CollectionOptions) -> Self {
        Self { options, ..self }
    }
}
