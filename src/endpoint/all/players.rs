use reqwest::{Method, Request, Response};
use url::Url;

use crate::{
    endpoint::{
        deserialize, sealed::Sealed, CollectionOptions, EndpointError, ListResponse, BASE_URL,
    },
    model::{player::Player, Identifier},
};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ListPlayers(pub CollectionOptions);

impl Sealed for ListPlayers {
    type Response = ListResponse<Player>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/players", BASE_URL))?;
        self.0.add_params(&mut url);
        Ok(Request::new(Method::GET, url))
    }

    async fn from_response(response: Response) -> Result<Self::Response, EndpointError> {
        ListResponse::from_response(response).await
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GetPlayer<'a>(pub Identifier<'a>);

impl Sealed for GetPlayer<'_> {
    type Response = Player;

    fn to_request(self) -> Result<Request, EndpointError> {
        let url = Url::parse(&format!("{}/players/{}", BASE_URL, self.0))?;
        Ok(Request::new(Method::GET, url))
    }

    async fn from_response(response: Response) -> Result<Self::Response, EndpointError> {
        deserialize(response.error_for_status()?).await
    }
}
