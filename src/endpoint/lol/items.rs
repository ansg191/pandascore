use reqwest::{Request, Response};
use url::Url;

use crate::{
    endpoint::{sealed::Sealed, EndpointError, BASE_URL},
    model::lol::item::Item,
};

crate::endpoint::list_endpoint!(ListItems("/lol/items") => Item);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GetItem(pub u64);

impl Sealed for GetItem {
    type Response = Item;

    fn to_request(self) -> Result<Request, EndpointError> {
        let url = Url::parse(&format!("{}/lol/items/{}", BASE_URL, self.0))?;
        Ok(Request::new(reqwest::Method::GET, url))
    }

    async fn from_response(response: Response) -> Result<Self::Response, EndpointError> {
        crate::endpoint::deserialize(response.error_for_status()?).await
    }
}
