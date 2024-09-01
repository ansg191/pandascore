use reqwest::{
    header::{HeaderValue, InvalidHeaderValue, ACCEPT, AUTHORIZATION},
    Error, Request, Response,
};
use tower::{Service, ServiceExt};

use crate::endpoint::{Endpoint, EndpointError};

#[derive(Debug, Clone)]
pub struct Client<T> {
    client: T,
    auth_header: HeaderValue,
}

impl<T> Client<T>
where
    T: Service<Request, Response = Response, Error = Error> + Clone,
{
    pub fn new(client: T, token: impl Into<String>) -> Result<Self, InvalidHeaderValue> {
        let auth_header = format!("Bearer {}", token.into()).parse()?;

        Ok(Self {
            client,
            auth_header,
        })
    }

    async fn execute_internal(&self, mut request: Request) -> Result<Response, Error> {
        request
            .headers_mut()
            .insert(AUTHORIZATION, self.auth_header.clone());
        request
            .headers_mut()
            .insert(ACCEPT, HeaderValue::from_static("application/json"));

        self.client.clone().oneshot(request).await
    }

    pub async fn execute<R>(&self, request: R) -> Result<R::Response, EndpointError>
    where
        R: Endpoint,
    {
        R::from_response(self.execute_internal(request.to_request()?).await?).await
    }
}
