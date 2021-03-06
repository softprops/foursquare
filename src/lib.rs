//! Foursquare provides a api bindings to the
//! [foursquare.com API](https://developer.foursquare.com/)
//!
//! # Examples
//!
//! Typical use will require instantiation of a `foursquare::Client`.
//! This requires a version string, set of `foursquare::Credentials`
//! and a tokio_core `Handle` reference.
//!
//! ```no_run
//! extern crate foursquare;
//! extern crate hyper;
//! extern crate tokio_core;
//!
//! use tokio_core::reactor::Core;
//! use foursquare::{Credentials, Client};
//!
//! fn main() {
//!   let mut core = Core::new().expect("reactor fail");
//!   let fs = Client::new(
//!     "YYYYMMDD",
//!     Credentials::client(
//!       "client_id", "client_secret"
//!     ),
//!     &core.handle()
//!   );
//! }
//! ```
//!
//! Access to various services are provided via methods on instances of
//! the `Client` type.
//!
//! The convention for executing operations typically looks like
//! `client.venues().operation(&OperationOptions)` where operation is the name
//! of the operation to perform
//!
//! # Errors
//!
//! Operations typically result in a `foursquare::Future` Type which is an alias
//! for the the [futures](https://docs.rs/futures/futures) crates Future trait
//! with the Error type fixed to the
//! [foursquare::Error](error/struct.Error.html) type.
//!
#![allow(missing_docs)] // todo: make this a deny eventually

#[macro_use]
extern crate derive_builder;
extern crate futures;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate hyper;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate url;
extern crate tokio_core;
#[cfg(feature = "tls")]
extern crate hyper_tls;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use futures::{Future as StdFuture, IntoFuture, Stream as StdStream};
use hyper::{Client as HyperClient, Method};
use hyper::client::{Connect, HttpConnector, Request};
#[cfg(feature = "tls")]
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use tokio_core::reactor::Handle;
use url::Url;

pub mod venue;
pub use venue::Venues;
pub mod error;
pub use error::{Error, ErrorKind, Result};

const DEFAULT_HOST: &str = "https://api.foursquare.com";

/// A type alias for `Futures` that may return `foursquare::Errors`
pub type Future<T> = Box<StdFuture<Item = T, Error = Error>>;

/// types of credentials used to authenticate requests
///
/// see [this doc](https://developer.foursquare.com/docs/api/configuration/authentication)
/// for more information
#[derive(Debug, PartialEq, Clone)]
pub enum Credentials {
    /// Userless authentication
    Client {
        client_id: String,
        client_secret: String,
    },
    /// User authentication, specific to a foursquare member
    User { oauth_token: String },
}

impl Credentials {
    /// Return a new set of Client credentials
    pub fn client<I, S>(id: I, secret: S) -> Self
    where
        I: Into<String>,
        S: Into<String>,
    {
        Credentials::Client {
            client_id: id.into(),
            client_secret: secret.into(),
        }
    }

    /// Return a new User credential
    pub fn user<T>(token: T) -> Self
    where
        T: Into<String>,
    {
        Credentials::User { oauth_token: token.into() }
    }
}

/// Entry point interface for interacting with Foursquare API
#[derive(Clone, Debug)]
pub struct Client<C>
where
    C: Clone + Connect,
{
    host: String,
    version: String,
    http: HyperClient<C>,
    credentials: Credentials,
}

#[cfg(feature = "tls")]
impl Client<HttpsConnector<HttpConnector>> {
    /// returns a new client
    ///
    /// version should be in `YYYYMMDD` format
    pub fn new<V>(version: V, credentials: Credentials, handle: &Handle) -> Self
    where
        V: Into<String>,
    {
        let connector = HttpsConnector::new(4, handle).unwrap();
        let http = HyperClient::configure()
            .connector(connector)
            .keep_alive(true)
            .build(handle);
        Self::custom(version, credentials, http)
    }
}

impl<C> Client<C>
where
    C: Clone + Connect,
{
    /// Return a new Client with a custom `hyper::Client`
    pub fn custom<V>(
        version: V,
        credentials: Credentials,
        http: HyperClient<C>,
    ) -> Self
    where
        V: Into<String>,
    {
        Self {
            host: DEFAULT_HOST.to_owned(),
            version: version.into(),
            http: http,
            credentials: credentials,
        }
    }

    /// Return an interface to venue operations
    pub fn venues(&self) -> Venues<C> {
        Venues::new(self.clone())
    }

    fn get<Out>(&self, uri: String) -> Future<Out>
    where
        Out: DeserializeOwned + 'static,
    {
        self.request(Method::Get, uri, None)
    }

    fn request<Out>(
        &self,
        method: Method,
        uri: String,
        body: Option<Vec<u8>>,
    ) -> Future<Out>
    where
        Out: DeserializeOwned + 'static,
    {
        let url = {
            let mut parsed = Url::parse(&uri).unwrap();
            parsed.query_pairs_mut().append_pair(
                "v",
                self.version.as_ref(),
            );
            if let Credentials::User { ref oauth_token } = self.credentials {
                parsed.query_pairs_mut().append_pair(
                    "oauth_token",
                    oauth_token.as_str(),
                );
            }
            if let Credentials::Client {
                ref client_id,
                ref client_secret,
            } = self.credentials
            {
                parsed
                    .query_pairs_mut()
                    .append_pair("client_id", client_id.as_str())
                    .append_pair("client_secret", client_secret.as_str());
            }
            parsed.to_string().parse().into_future()
        };
        let instance = self.clone();
        let body2 = body.clone();
        let method2 = method.clone();
        let response = url.map_err(Error::from).and_then(move |url| {
            let mut req = Request::new(method2, url);

            if let Some(body) = body2 {
                req.set_body(body)
            }
            instance.http.request(req).map_err(Error::from)
        });
        Box::new(response.and_then(move |response| {
            debug!("response headers {:?}", response.headers());
            let status = response.status();
            Box::new(response.body().concat2().map_err(Error::from).and_then(
                move |response_body| if status.is_success() {
                    debug!(
                        "response payload {}",
                        String::from_utf8_lossy(&response_body)
                    );
                    serde_json::from_slice::<Out>(&response_body).map_err(
                        |error| {
                            ErrorKind::Codec(error).into()
                        },
                    )
                } else {
                    debug!(
                        "response error {}",
                        String::from_utf8_lossy(&response_body)
                    );
                    Err(
                        ErrorKind::Fault {
                            code: status,
                            error: serde_json::from_slice(&response_body)?,
                        }.into(),
                    )
                },
            ))
        }))
    }
}

// representations

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Meta {
    pub code: u16,
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[serde(rename = "errorType")]
    pub error_type: Option<String>,
    #[serde(rename = "errorDetail")]
    pub error_detail: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Response<T> {
    pub meta: Meta,
    pub response: T,
}