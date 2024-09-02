use reqwest::{Request, Response};
use url::Url;

use crate::{
    endpoint::{sealed::Sealed, EndpointError, BASE_URL},
    model::{
        bracket::{TournamentBracket, TournamentBracketMatch},
        Identifier,
    },
};

crate::endpoint::get_endpoint!(GetTournament("/tournaments") => crate::model::tournament::Tournament);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GetTournamentBracket<'a>(pub Identifier<'a>);

//TODO: add pagination
impl Sealed for GetTournamentBracket<'_> {
    type Response = TournamentBracket;

    fn to_request(self) -> Result<Request, EndpointError> {
        let url = Url::parse(&format!("{}/tournaments/{}/brackets", BASE_URL, self.0))?;
        Ok(Request::new(reqwest::Method::GET, url))
    }

    async fn from_response(response: Response) -> Result<Self::Response, EndpointError> {
        let bracket: Vec<TournamentBracketMatch> =
            crate::endpoint::deserialize(response.error_for_status()?).await?;
        Ok(TournamentBracket::new(bracket))
    }
}
