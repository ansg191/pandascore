// Due to bon: fix these later
#![allow(private_bounds, clippy::missing_const_for_fn)]

use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
    sync::LazyLock,
};

use compact_str::{format_compact, CompactString, CompactStringExt, ToCompactString};
use linkify::{Link, LinkFinder, LinkKind};
use regex::Regex;
use reqwest::header::{AsHeaderName, LINK};
use serde::de::DeserializeOwned;

pub mod all;
pub mod lol;
pub mod rl;

const BASE_URL: &str = "https://api.pandascore.co";

mod sealed {
    use std::future::Future;

    use crate::endpoint::EndpointError;

    pub trait Sealed {
        type Response;

        fn to_request(self) -> Result<reqwest::Request, EndpointError>;
        fn from_response(
            response: reqwest::Response,
        ) -> impl Future<Output = Result<Self::Response, EndpointError>> + Send;
    }
}

pub trait Endpoint: sealed::Sealed {}

impl<T: sealed::Sealed> Endpoint for T {}

async fn deserialize<T: DeserializeOwned>(response: reqwest::Response) -> Result<T, EndpointError> {
    let body = response.bytes().await?;
    let mut jd = serde_json::Deserializer::from_slice(body.as_ref());
    Ok(serde_path_to_error::deserialize(&mut jd)?)
}

#[derive(Debug, thiserror::Error)]
pub enum EndpointError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Serde(#[from] serde_path_to_error::Error<serde_json::Error>),
    #[error(transparent)]
    UrlParse(#[from] url::ParseError),
    #[error("Failed to convert header to string: {0}")]
    ToStr(#[from] reqwest::header::ToStrError),
    #[error("Failed to parse integer: {0}")]
    InvalidInt(#[from] std::num::ParseIntError),
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct CollectionOptions {
    /// <https://developers.pandascore.co/docs/filtering-and-sorting#filter>
    filters: HashMap<CompactString, Vec<CompactString>>,
    /// <https://developers.pandascore.co/docs/filtering-and-sorting#search>
    search: HashMap<CompactString, CompactString>,
    /// <https://developers.pandascore.co/docs/filtering-and-sorting#range>
    range: HashMap<CompactString, (i64, i64)>,
    /// <https://developers.pandascore.co/docs/filtering-and-sorting#sort>
    sort: HashSet<CompactString>,

    /// <https://developers.pandascore.co/docs/pagination#page-number>
    page: Option<u32>,
    /// <https://developers.pandascore.co/docs/pagination#page-size>
    per_page: Option<u32>,
}

impl CollectionOptions {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn filter(
        mut self,
        key: impl Into<CompactString>,
        value: impl Into<CompactString>,
    ) -> Self {
        self.filters
            .entry(key.into())
            .or_default()
            .push(value.into());
        self
    }

    #[must_use]
    pub fn search(
        mut self,
        key: impl Into<CompactString>,
        value: impl Into<CompactString>,
    ) -> Self {
        self.search.insert(key.into(), value.into());
        self
    }

    #[must_use]
    pub fn range(mut self, key: impl Into<CompactString>, start: i64, end: i64) -> Self {
        self.range.insert(key.into(), (start, end));
        self
    }

    #[must_use]
    pub fn sort(mut self, key: impl Into<CompactString>) -> Self {
        self.sort.insert(key.into());
        self
    }

    #[must_use]
    pub const fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    #[must_use]
    pub const fn per_page(mut self, per_page: u32) -> Self {
        self.per_page = Some(per_page);
        self
    }

    fn add_params(self, url: &mut url::Url) {
        let mut query = url.query_pairs_mut();

        for (key, values) in self.filters {
            let key = format_compact!("filter[{}]", key);
            let value = values.join_compact(",");
            query.append_pair(&key, &value);
        }

        for (key, value) in self.search {
            let key = format_compact!("search[{}]", key);
            query.append_pair(&key, &value);
        }

        for (key, (start, end)) in self.range {
            let key = format_compact!("range[{}]", key);
            let value = format!("{start},{end}");
            query.append_pair(&key, &value);
        }

        if !self.sort.is_empty() {
            let value = self.sort.join_compact(",");
            query.append_pair("sort", &value);
        }

        if let Some(page) = self.page {
            query.append_pair("page[number]", &page.to_compact_string());
        }
        if let Some(per_page) = self.per_page {
            query.append_pair("page[size]", &per_page.to_compact_string());
        }
    }

    fn from_url(url: &str) -> Result<Self, EndpointError> {
        let url = url::Url::parse(url)?;
        let query = url.query_pairs();

        let mut ret = Self::default();
        for (key, value) in query {
            let Some(captures) = KEY_REGEX.captures(&key) else {
                continue;
            };

            match &captures[1] {
                "filter" => {
                    let Some(key) = captures.get(3) else {
                        continue;
                    };
                    let key = key.as_str().to_compact_string();
                    let value = value.split(',').map(CompactString::from).collect();
                    ret.filters.insert(key, value);
                }
                "search" => {
                    let Some(key) = captures.get(3) else {
                        continue;
                    };
                    ret.search
                        .insert(key.as_str().to_compact_string(), value.to_compact_string());
                }
                "range" => {
                    let Some(key) = captures.get(3) else {
                        continue;
                    };
                    let key = key.as_str().to_compact_string();
                    let Some((start, end)) = value.split_once(',') else {
                        continue;
                    };
                    let start = start.parse()?;
                    let end = end.parse()?;
                    ret.range.insert(key, (start, end));
                }
                "sort" => ret.sort = value.split(',').map(CompactString::from).collect(),
                "page" => {
                    if let Some(tp) = captures.get(3) {
                        match tp.as_str() {
                            "number" => ret.page = Some(value.parse()?),
                            "size" => ret.per_page = Some(value.parse()?),
                            _ => continue,
                        }
                    } else {
                        ret.page = Some(value.parse()?);
                    }
                }
                "per_page" => ret.per_page = Some(value.parse()?),
                _ => continue,
            }
        }

        Ok(ret)
    }
}

static KEY_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    // Matches "filter[...]", "search[...]", "range[...]", "sort"
    // https://regex101.com/r/ZPylAq/1
    Regex::new(r"([a-z]+)(\[(.+)])?").unwrap()
});

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListResponse<T> {
    pub results: Vec<T>,
    pub total: u64,
    pub next: Option<CollectionOptions>,
    pub prev: Option<CollectionOptions>,
}

static LINK_REL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"rel="([a-z]+)""#).unwrap());

impl<T: DeserializeOwned> ListResponse<T> {
    async fn from_response(response: reqwest::Response) -> Result<Self, EndpointError> {
        let response = response.error_for_status()?;

        let total = parse_header_int(&response, "X-Total")?.unwrap_or(0);
        let link_str = response
            .headers()
            .get(LINK)
            .map(|v| v.to_str())
            .transpose()?;

        let Some(link_str) = link_str else {
            return Ok(Self {
                results: deserialize(response).await?,
                total,
                next: None,
                prev: None,
            });
        };

        let mut next = None;
        let mut prev = None;

        // Format:
        // <https://some.url>; rel="x",
        // where x is first, last, next, or prev
        let mut finder = LinkFinder::new();
        finder.kinds(&[LinkKind::Url]);
        let links = finder.links(link_str).collect::<Vec<Link>>();

        for (i, link) in links.iter().enumerate() {
            // Find `rel=` attribute between this link and the next
            let substr = &link_str
                [link.start()..links.get(i + 1).map_or_else(|| link_str.len(), Link::start)];

            let Some(captures) = LINK_REL_REGEX.captures(substr) else {
                // No `rel=` attribute found
                continue;
            };
            match &captures[1] {
                "next" => next = Some(CollectionOptions::from_url(link.as_str())?),
                "prev" => prev = Some(CollectionOptions::from_url(link.as_str())?),
                _ => continue,
            }
        }

        Ok(Self {
            results: deserialize(response).await?,
            total,
            next,
            prev,
        })
    }
}

impl<T> Deref for ListResponse<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.results
    }
}

impl<T> DerefMut for ListResponse<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.results
    }
}

fn parse_header_int<K, T>(
    response: &reqwest::Response,
    header: K,
) -> Result<Option<T>, EndpointError>
where
    K: AsHeaderName,
    T: std::str::FromStr<Err = std::num::ParseIntError>,
{
    Ok(response
        .headers()
        .get(header)
        .map(|v| v.to_str())
        .transpose()?
        .map(str::parse)
        .transpose()?)
}

macro_rules! game_endpoints {
    ($endpoint:literal) => {
        pub mod leagues {
            $crate::endpoint::list_endpoint!(
                ListLeagues(concat!("/", $endpoint, "/leagues")) => $crate::model::league::League
            );
        }
        pub mod matches {
            $crate::endpoint::multi_list_endpoint!(
                ListMatches(concat!("/", $endpoint, "/matches")) => $crate::model::matches::Match
            );
        }
        pub mod players {
            $crate::endpoint::list_endpoint!(
                ListPlayers(concat!("/", $endpoint, "/players")) => $crate::model::player::Player
            );
        }
        pub mod series {
            $crate::endpoint::multi_list_endpoint!(
                ListSeries(concat!("/", $endpoint, "/series")) => $crate::model::series::Series
            );
        }
        pub mod teams {
            $crate::endpoint::list_endpoint!(
                ListTeams(concat!("/", $endpoint, "/teams")) => $crate::model::team::Team
            );
        }
        pub mod tournaments {
            $crate::endpoint::multi_list_endpoint!(
                ListTournaments(concat!("/", $endpoint, "/tournaments")) => $crate::model::tournament::Tournament
            );
        }
    };
}
pub(crate) use game_endpoints;

macro_rules! get_endpoint {
    ($name:ident($path:expr) => $response:ty) => {
        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct $name<'a>(pub $crate::model::Identifier<'a>);

        impl<'a> $crate::endpoint::sealed::Sealed for $name<'a> {
            type Response = $response;

            fn to_request(
                self,
            ) -> std::result::Result<::reqwest::Request, $crate::endpoint::EndpointError> {
                let url = ::url::Url::parse(&format!(
                    concat!("{}", $path, "/{}"),
                    $crate::endpoint::BASE_URL,
                    self.0
                ))?;
                Ok(::reqwest::Request::new(::reqwest::Method::GET, url))
            }

            async fn from_response(
                response: ::reqwest::Response,
            ) -> ::std::result::Result<Self::Response, $crate::endpoint::EndpointError> {
                $crate::endpoint::deserialize(response.error_for_status()?).await
            }
        }

        impl<'a, T> ::std::convert::From<T> for $name<'a>
        where
            T: Into<$crate::model::Identifier<'a>>,
        {
            fn from(id: T) -> Self {
                Self(id.into())
            }
        }
    };
}
pub(crate) use get_endpoint;

macro_rules! list_endpoint {
    ($name:ident($path:expr) => $response:ty) => {
        #[derive(Debug, Clone, Eq, PartialEq, Default)]
        pub struct $name(pub $crate::endpoint::CollectionOptions);

        impl $crate::endpoint::sealed::Sealed for $name {
            type Response = $crate::endpoint::ListResponse<$response>;

            fn to_request(
                self,
            ) -> std::result::Result<::reqwest::Request, $crate::endpoint::EndpointError> {
                let mut url =
                    ::url::Url::parse(&format!(concat!("{}", $path), $crate::endpoint::BASE_URL))?;
                self.0.add_params(&mut url);
                Ok(::reqwest::Request::new(::reqwest::Method::GET, url))
            }

            fn from_response(
                response: ::reqwest::Response,
            ) -> impl ::std::future::Future<
                Output = ::std::result::Result<Self::Response, $crate::endpoint::EndpointError>,
            > + Send {
                $crate::endpoint::ListResponse::from_response(response)
            }
        }
    };
}
pub(crate) use list_endpoint;

macro_rules! multi_list_endpoint {
    ($name:ident($path:expr) => $response:ty) => {
        #[::bon::builder]
        #[derive(Debug, Clone, Eq, PartialEq, Default)]
        pub struct $name {
            pub status: ::std::option::Option<$crate::model::EventStatus>,
            #[builder(default)]
            pub options: $crate::endpoint::CollectionOptions,
        }

        impl $crate::endpoint::sealed::Sealed for $name {
            type Response = $crate::endpoint::ListResponse<$response>;

            fn to_request(
                self,
            ) -> std::result::Result<::reqwest::Request, $crate::endpoint::EndpointError> {
                let mut url = ::url::Url::parse(&format!(
                    concat!("{}", $path, "/"),
                    $crate::endpoint::BASE_URL
                ))?;
                self.options.add_params(&mut url);
                if let Some(status) = self.status {
                    url = url.join(status.as_str())?;
                }
                Ok(::reqwest::Request::new(::reqwest::Method::GET, url))
            }

            fn from_response(
                response: ::reqwest::Response,
            ) -> impl ::std::future::Future<
                Output = ::std::result::Result<Self::Response, $crate::endpoint::EndpointError>,
            > + Send {
                $crate::endpoint::ListResponse::from_response(response)
            }
        }
    };
}
pub(crate) use multi_list_endpoint;

#[cfg(test)]
mod tests {
    use url::Url;

    use super::*;

    #[test]
    fn test_collection_options_add_params() {
        let mut url = Url::parse("https://example.com").unwrap();

        let options = CollectionOptions::new()
            .filter("foo", "bar")
            .filter("foo", "baz")
            .filter("qux", "quux")
            .search("qux", "quux")
            .range("corge", 1, 5)
            .sort("grault")
            .sort("-garply")
            .page(3)
            .per_page(4);
        options.clone().add_params(&mut url);

        assert!(url.query().is_some());

        let options2 = CollectionOptions::from_url(url.as_str()).unwrap();
        assert_eq!(options, options2);
    }
}
