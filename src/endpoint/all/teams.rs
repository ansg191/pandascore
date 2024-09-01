use reqwest::{Request, Response};
use url::Url;

use crate::{
    endpoint::{sealed::Sealed, CollectionOptions, EndpointError, ListResponse, BASE_URL},
    model::{team::Team, Identifier},
};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ListTeams(pub CollectionOptions);

impl Sealed for ListTeams {
    type Response = ListResponse<Team>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/teams", BASE_URL))?;
        self.0.add_params(&mut url);
        Ok(Request::new(reqwest::Method::GET, url))
    }

    async fn from_response(response: Response) -> Result<Self::Response, EndpointError> {
        ListResponse::from_response(response).await
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GetTeam<'a>(pub Identifier<'a>);

impl Sealed for GetTeam<'_> {
    type Response = Team;

    fn to_request(self) -> Result<Request, EndpointError> {
        let url = Url::parse(&format!("{}/teams/{}", BASE_URL, self.0))?;
        Ok(Request::new(reqwest::Method::GET, url))
    }

    async fn from_response(response: Response) -> Result<Self::Response, EndpointError> {
        crate::endpoint::deserialize(response.error_for_status()?).await
    }
}
