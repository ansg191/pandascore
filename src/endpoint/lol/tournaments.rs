use std::future::Future;

use derive_builder::Builder;
use reqwest::{Request, Response};
use url::Url;

use crate::{
    endpoint::{sealed::Sealed, CollectionOptions, EndpointError, ListResponse, BASE_URL},
    model::{tournament::Tournament, EventStatus},
};

#[derive(Debug, Clone, PartialEq, Eq, Default, Builder)]
#[builder(build_fn(error = "crate::endpoint::BuilderError"))]
pub struct ListTournaments {
    #[builder(default)]
    status: Option<EventStatus>,
    #[builder(default)]
    options: CollectionOptions,
}

impl ListTournaments {
    pub fn builder() -> ListTournamentsBuilder {
        ListTournamentsBuilder::create_empty()
    }
}

impl Sealed for ListTournaments {
    type Response = ListResponse<Tournament>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/lol/tournaments/", BASE_URL))?;
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
