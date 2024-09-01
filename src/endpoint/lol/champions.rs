use reqwest::{Request, Response};
use url::Url;

use crate::{
    endpoint::{sealed::Sealed, CollectionOptions, EndpointError, ListResponse, BASE_URL},
    model::lol::champion::Champion,
};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ListChampions(pub CollectionOptions);

impl Sealed for ListChampions {
    type Response = ListResponse<Champion>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/champions", BASE_URL))?;
        self.0.add_params(&mut url);
        Ok(Request::new(reqwest::Method::GET, url))
    }

    async fn from_response(response: Response) -> Result<Self::Response, EndpointError> {
        ListResponse::from_response(response).await
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GetChampion(pub u64);

impl Sealed for GetChampion {
    type Response = Champion;

    fn to_request(self) -> Result<Request, EndpointError> {
        let url = Url::parse(&format!("{}/champions/{}", BASE_URL, self.0))?;
        Ok(Request::new(reqwest::Method::GET, url))
    }

    async fn from_response(response: Response) -> Result<Self::Response, EndpointError> {
        crate::endpoint::deserialize(response.error_for_status()?).await
    }
}
