use reqwest::{Request, Response};
use url::Url;

use crate::{
    endpoint::{sealed::Sealed, CollectionOptions, EndpointError, ListResponse, BASE_URL},
    model::{matches::MatchOpponents, team::Team, Identifier},
};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ListMatches(pub CollectionOptions);

impl Sealed for ListMatches {
    type Response = ListResponse<Team>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/matches", BASE_URL))?;
        self.0.add_params(&mut url);
        Ok(Request::new(reqwest::Method::GET, url))
    }

    async fn from_response(response: Response) -> Result<Self::Response, EndpointError> {
        ListResponse::from_response(response).await
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GetMatch<'a>(pub Identifier<'a>);

impl Sealed for GetMatch<'_> {
    type Response = Team;

    fn to_request(self) -> Result<Request, EndpointError> {
        let url = Url::parse(&format!("{}/matches/{}", BASE_URL, self.0))?;
        Ok(Request::new(reqwest::Method::GET, url))
    }

    async fn from_response(response: Response) -> Result<Self::Response, EndpointError> {
        crate::endpoint::deserialize(response.error_for_status()?).await
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GetMatchOpponents<'a>(pub Identifier<'a>);

impl Sealed for GetMatchOpponents<'_> {
    type Response = MatchOpponents;

    fn to_request(self) -> Result<Request, EndpointError> {
        let url = Url::parse(&format!("{}/matches/{}/opponents", BASE_URL, self.0))?;
        Ok(Request::new(reqwest::Method::GET, url))
    }

    async fn from_response(response: Response) -> Result<Self::Response, EndpointError> {
        crate::endpoint::deserialize(response.error_for_status()?).await
    }
}
