#![allow(missing_docs)] // todo: make this a deny eventually

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
extern crate url;
extern crate tokio_core;
#[cfg(feature = "tls")]
extern crate hyper_tls;

use futures::{Future as StdFuture, IntoFuture, Stream as StdStream};
use hyper::{Client as HyperClient, Method};
use hyper::client::{Connect, HttpConnector, Request};
#[cfg(feature = "tls")]
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use tokio_core::reactor::Handle;
use url::Url;

pub mod venues;
pub use venues::Venues;
pub mod errors;
pub use errors::{Error, ErrorKind, Result};

const DEFAULT_HOST: &str = "https://api.foursquare.com";

/// A type alias for `Futures` that may return `foursquare::Errors`
pub type Future<T> = Box<StdFuture<Item = T, Error = Error>>;

/// A type alias for `Streams` that may result in `foursquare::Errors`
pub type Stream<T> = Box<StdStream<Item = T, Error = Error>>;


#[derive(Debug, PartialEq, Clone)]
pub struct Credentials {
    client_id: String,
    client_secret: String,
}

impl Credentials {
    pub fn new<I, S>(id: I, secret: S) -> Self
    where
        I: Into<String>,
        S: Into<String>,
    {
        Credentials {
            client_id: id.into(),
            client_secret: secret.into(),
        }
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
    credentials: Option<Credentials>,
}

#[cfg(feature = "tls")]
impl Client<HttpsConnector<HttpConnector>> {
    pub fn new<A>(
        version: A,
        credentials: Option<Credentials>,
        handle: &Handle,
    ) -> Self
    where
        A: Into<String>,
    {
        Self::host(DEFAULT_HOST, version, credentials, handle)
    }

    pub fn host<H, A>(
        host: H,
        version: A,
        credentials: Option<Credentials>,
        handle: &Handle,
    ) -> Self
    where
        H: Into<String>,
        A: Into<String>,
    {
        let connector = HttpsConnector::new(4, handle).unwrap();
        let http = HyperClient::configure()
            .connector(connector)
            .keep_alive(true)
            .build(handle);
        Self::custom(host, version, credentials, http)
    }
}

impl<C> Client<C>
where
    C: Clone + Connect,
{
    pub fn custom<H, A>(
        host: H,
        version: A,
        credentials: Option<Credentials>,
        http: HyperClient<C>,
    ) -> Self
    where
        H: Into<String>,
        A: Into<String>,
    {
        Self {
            host: host.into(),
            version: version.into(),
            http: http,
            credentials: credentials,
        }
    }

    pub fn venues(&self) -> Venues<C> {
        Venues::new(self.clone())
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
            if let Some(Credentials {
                            ref client_id,
                            ref client_secret,
                        }) = self.credentials
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
