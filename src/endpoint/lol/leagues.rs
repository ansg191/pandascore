use std::future::Future;

use reqwest::{Request, Response};
use url::Url;
use crate::{
    endpoint::{sealed::Sealed, CollectionOptions, EndpointError, ListResponse},
    model::league::League,
};
use crate::endpoint::BASE_URL;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ListLeagues(pub CollectionOptions);

impl Sealed for ListLeagues {
    type Response = ListResponse<League>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/lol/leagues", BASE_URL))?;
        self.0.add_params(&mut url);
        Ok(Request::new(reqwest::Method::GET, url))
    }

    fn from_response(
        response: Response,
    ) -> impl Future<Output = Result<Self::Response, EndpointError>> + Send {
        ListResponse::from_response(response)
    }
}
