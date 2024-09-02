#![allow(dead_code)]
use std::{
    future::Ready,
    task::{Context, Poll},
};

use http::StatusCode;
use reqwest::{Request, Response};
use tower::Service;

#[derive(Debug, Clone)]
pub struct MockClient {
    expectations: Vec<Expectation>,
    response: &'static [u8],
}

impl MockClient {
    #[must_use]
    pub const fn new(response: &'static [u8]) -> Self {
        Self {
            expectations: Vec::new(),
            response,
        }
    }

    #[must_use]
    pub fn expect(mut self, expectation: Expectation) -> Self {
        self.expectations.push(expectation);
        self
    }
}

impl Service<Request> for MockClient {
    type Response = Response;
    type Error = reqwest::Error;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        for expectation in &self.expectations {
            expectation.validate(&req);
        }

        let response = http::Response::builder()
            .status(StatusCode::OK)
            .body(self.response)
            .unwrap();

        std::future::ready(Ok(response.into()))
    }
}

#[derive(Debug, Clone)]
pub enum Expectation {
    Method(reqwest::Method),
    Path(&'static str),
    Query(&'static str, &'static str),
}

impl Expectation {
    fn validate(&self, req: &Request) {
        match self {
            Self::Method(method) => assert_eq!(req.method(), method),
            Self::Path(path) => assert_eq!(req.url().path(), *path),
            Self::Query(key, value) => {
                let query = req
                    .url()
                    .query_pairs()
                    .collect::<std::collections::HashMap<_, _>>();
                assert_eq!(query[*key], *value);
            }
        }
    }
}
