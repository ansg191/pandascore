use std::future::Future;

use derive_builder::Builder;
use reqwest::{Request, Response};
use url::Url;

use crate::{
    endpoint::{sealed::Sealed, CollectionOptions, EndpointError, ListResponse, BASE_URL},
    model::{series::Series, EventStatus},
};

#[derive(Debug, Clone, PartialEq, Eq, Default, Builder)]
#[builder(build_fn(error = "crate::endpoint::BuilderError"))]
pub struct ListSeries {
    #[builder(default)]
    status: Option<EventStatus>,
    #[builder(default)]
    options: CollectionOptions,
}

impl ListSeries {
    pub fn builder() -> ListSeriesBuilder {
        ListSeriesBuilder::create_empty()
    }
}

impl Sealed for ListSeries {
    type Response = ListResponse<Series>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/lol/series/", BASE_URL))?;
        self.options.add_params(&mut url);
        if let Some(status) = self.status {
            url = url.join(status.as_str())?;
        }
        Ok(Request::new(reqwest::Method::GET, url))
    }

    fn from_response(
        response: Response,
    ) -> impl Future<Output = Result<Self::Response, EndpointError>> + Send {
        ListResponse::from_response(response)
    }
}
