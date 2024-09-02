use reqwest::{Request, Response};
use url::Url;

use crate::{
    endpoint::{sealed::Sealed, EndpointError, BASE_URL},
    model::lol::spell::Spell,
};

crate::endpoint::list_endpoint!(ListSpells("/lol/spells") => Spell);

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
