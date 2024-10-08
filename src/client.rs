use reqwest::{
    header::{HeaderValue, InvalidHeaderValue, ACCEPT, AUTHORIZATION},
    Error, Request, Response,
};
use tower::{Service, ServiceExt};

use crate::endpoint::{Endpoint, EndpointError};

/// A trait for the underlying HTTP client implementation.
pub trait ClientTransport: Service<Request, Response = Response, Error = Error> + Clone {}
impl<T> ClientTransport for T where T: Service<Request, Response = Response, Error = Error> + Clone {}

/// A client that can execute requests to the API.
///
/// The client is generic over the underlying HTTP client implementation.
/// Currently, the underlying client must use reqwest's [`Request`] and return reqwest's
/// [`Response`].
/// However, this could be changed to use a different client implementation using the `http` crate
/// in the future,
/// especially when [this reqwest issue](https://github.com/seanmonstar/reqwest/issues/2251) is
/// resolved.
#[derive(Debug, Clone)]
pub struct Client<T> {
    client: T,
    auth_header: HeaderValue,
}

#[allow(clippy::future_not_send)]
impl<T: ClientTransport> Client<T> {
    /// Create a new client with the given underlying client and token.
    ///
    /// # Arguments
    ///
    /// * `client`: the underlying HTTP client implementation.
    /// * `token`: the `PandaScore` token to use for authentication.
    ///
    /// Returns: `Result<Client<T>, InvalidHeaderValue>`
    ///
    /// # Errors
    ///
    /// Returns an error if the token doesn't consist of **only** visible ASCII characters (32-127).
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

    /// Execute the given request and return the response.
    ///
    /// # Arguments
    ///
    /// * `request`: a endpoint request to execute.
    ///
    /// Returns: `Result<<R as Sealed>::Response, EndpointError>`
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use pandascore::{Client, endpoint::all::players::GetPlayer};
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new(reqwest::Client::new(), "token").unwrap();
    /// let player = client.execute(GetPlayer::from("faker")).await.unwrap();
    /// println!("{:?}", player);
    /// # }
    /// ```
    pub async fn execute<R>(&self, request: R) -> Result<R::Response, EndpointError>
    where
        R: Endpoint,
    {
        R::from_response(self.execute_internal(request.to_request()?).await?).await
    }
}
