use reqwest::{Request, Response};
use url::Url;

use crate::{
    endpoint::{sealed::Sealed, EndpointError, BASE_URL},
    model::{
        matches::{Match, MatchOpponents},
        Identifier,
    },
};

crate::endpoint::multi_list_endpoint!(ListMatches("/matches") => Match);
crate::endpoint::get_endpoint!(GetMatch("/matches") => Match);

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

impl<'a, T> From<T> for GetMatchOpponents<'a>
where
    T: Into<Identifier<'a>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
