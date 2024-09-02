use derive_builder::Builder;
use reqwest::{Method, Request, Response};
use url::Url;

use crate::{
    endpoint::{sealed::Sealed, CollectionOptions, EndpointError, ListResponse, BASE_URL},
    model::{league::League, matches::Match, series::Series, EventStatus, Identifier},
};

crate::endpoint::list_endpoint!(ListLeagues("/leagues") => League);
crate::endpoint::get_endpoint!(GetLeague("/leagues") => League);

#[derive(Debug, Clone, PartialEq, Eq, Builder)]
#[builder(build_fn(error = "crate::endpoint::BuilderError"))]
pub struct GetLeagueMatches<'a> {
    #[builder(setter(into))]
    id: Identifier<'a>,
    #[builder(default)]
    status: Option<EventStatus>,
    #[builder(default)]
    options: CollectionOptions,
}

impl GetLeagueMatches<'_> {
    pub fn builder() -> GetLeagueMatchesBuilder<'static> {
        GetLeagueMatchesBuilder::create_empty()
    }
}

impl Sealed for GetLeagueMatches<'_> {
    type Response = ListResponse<Match>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/leagues/{}/matches/", BASE_URL, self.id,))?;
        self.options.add_params(&mut url);
        if let Some(status) = self.status {
            url = url.join(status.as_str())?;
        }
        Ok(Request::new(Method::GET, url))
    }

    async fn from_response(response: Response) -> Result<Self::Response, EndpointError> {
        ListResponse::from_response(response).await
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Builder)]
#[builder(build_fn(error = "crate::endpoint::BuilderError"))]
pub struct ListLeagueSeries<'a> {
    #[builder(setter(into))]
    id: Identifier<'a>,
    #[builder(default)]
    options: CollectionOptions,
}

impl ListLeagueSeries<'_> {
    pub fn builder() -> ListLeagueSeriesBuilder<'static> {
        ListLeagueSeriesBuilder::create_empty()
    }
}

impl Sealed for ListLeagueSeries<'_> {
    type Response = ListResponse<Series>;

    fn to_request(self) -> Result<Request, EndpointError> {
        let mut url = Url::parse(&format!("{}/leagues/{}/series", BASE_URL, self.id,))?;
        self.options.add_params(&mut url);
        Ok(Request::new(Method::GET, url))
    }

    async fn from_response(response: Response) -> Result<Self::Response, EndpointError> {
        ListResponse::from_response(response).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_league() {
        let id = 1;
        let slug = "slug";
        let get_league_id = GetLeague(Identifier::Id(id));
        let get_league_slug = GetLeague(Identifier::Slug(slug));

        let request_id = get_league_id.to_request().unwrap();
        let request_slug = get_league_slug.to_request().unwrap();

        assert_eq!(request_id.method(), &Method::GET);
        assert_eq!(
            request_id.url().as_str(),
            "https://api.pandascore.co/leagues/1"
        );

        assert_eq!(request_slug.method(), &Method::GET);
        assert_eq!(
            request_slug.url().as_str(),
            "https://api.pandascore.co/leagues/slug"
        );
    }
}
