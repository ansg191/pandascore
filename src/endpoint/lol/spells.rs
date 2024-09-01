use std::future::Future;

use reqwest::{Request, Response};
use url::Url;

use crate::{
    endpoint::{sealed::Sealed, CollectionOptions, EndpointError, ListResponse, BASE_URL},
    model::lol::spell::Spell,
};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ListSpells(pub CollectionOptions);

impl Sealed for ListSpells {
    type Response = ListResponse<Spell>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/lol/spells", BASE_URL))?;
        self.0.add_params(&mut url);
        Ok(Request::new(reqwest::Method::GET, url))
    }

    fn from_response(
        response: Response,
    ) -> impl Future<Output = Result<Self::Response, EndpointError>> + Send {
        ListResponse::from_response(response)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GetSpell(pub u64);

impl Sealed for GetSpell {
    type Response = Spell;

    fn to_request(self) -> Result<Request, EndpointError> {
        let url = Url::parse(&format!("{}/lol/spells/{}", BASE_URL, self.0))?;
        Ok(Request::new(reqwest::Method::GET, url))
    }

    async fn from_response(response: Response) -> Result<Self::Response, EndpointError> {
        crate::endpoint::deserialize(response.error_for_status()?).await
    }
}
