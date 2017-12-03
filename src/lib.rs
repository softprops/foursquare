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

use futures::{Stream as StdStream, Future as StdFuture, IntoFuture};
#[cfg(feature = "tls")]
use hyper_tls::HttpsConnector;
use hyper::{Client as HyperClient, Method};
use hyper::client::{Connect, HttpConnector, Request};
use serde::de::DeserializeOwned;
use tokio_core::reactor::Handle;
use url::Url;
use url::form_urlencoded;

pub mod errors;
pub use errors::{Error, ErrorKind, Result};

const DEFAULT_HOST: &str = "https://api.foursquare.com";

/// A type alias for `Futures` that may return `github::Errors`
pub type Future<T> = Box<StdFuture<Item = T, Error = Error>>;

/// A type alias for `Streams` that may result in `github::Errors`
pub type Stream<T> = Box<StdStream<Item = T, Error = Error>>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Meta {
    pub code: u16,
    #[serde(rename = "requestId")]
    pub request_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T> {
    pub meta: Meta,
    pub response: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Coords {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Contact {
    pub phone: Option<String>,
    #[serde(rename = "formattedPhone")]
    pub formatted_phone: Option<String>,
    pub twitter: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Price {
    pub tier: u16,
    pub message: String,
    pub currency: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub address: String,
    #[serde(rename = "crossStreet")]
    pub cross_street: Option<String>,
    pub lat: f64,
    pub lng: f64,
    pub distance: Option<u32>,
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    pub cc: String,
    pub city: String,
    pub state: String,
    pub country: String,
    #[serde(rename = "formattedAddress")]
    pub formatted_address: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Icon {
    pub prefix: String,
    pub suffix: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    #[serde(rename = "pluralName")]
    pub plural_name: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    pub icon: Icon,
    pub primary: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Menu {
    pub label: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Venue {
    /// A unique string identifier for this venue.
    pub id: String,
    /// The best known name for this venue.
    pub name: String,
    /// An object containing none, some, or all of twitter, phone, and formattedPhone. All are strings.
    pub contact: Contact,
    /// An object containing none, some, or all of address (street address), crossStreet, city, state, postalCode, country, lat, lng, and distance. All fields are strings, except for lat, lng, and distance. Distance is measured in meters. Some venues have their locations intentionally hidden for privacy reasons (such as private residences). If this is the case, the parameter isFuzzed will be set to true, and the lat/lng parameters will have reduced precision.
    pub location: Location,
    /// An array, possibly empty, of categories that have been applied to this venue. One of the categories will have a primary field indicating that it is the primary category for the venue. For the complete category tree, see categories.
    pub categories: Vec<Category>,
    /// Boolean indicating whether the owner of this business has claimed it and verified the information.
    pub verified: bool,
    // Contains checkinsCount (total checkins ever here), usersCount (total users who have ever checked in here), and tipCount (number of tips here).
    // pub stats: Stats
    /// URL of the venue’s website, typically provided by the venue manager.
    pub url: Option<String>,

    // Contains the hours during the week that the venue is open along with any named hours segments in a human-readable format. For machine readable hours see venues/hours
    // pub hours: Option<Hours>,
    // Contains the hours during the week when people usually go to the venue. For machine readable hours see venues/hours.
    // pub popular: Hours
    #[serde(rename = "hasMenu")]
    pub has_menu: Option<bool>,
    /// An object containing url and mobileUrl that display the menu information for this venue.
    pub menu: Option<Menu>,
    /// An object containing the price tier from 1 (least pricey) - 4 (most pricey) and a message describing the price tier.
    pub price: Option<Price>,
    // Numerical rating of the venue (0 through 10). Not all venues will have a rating.
    // pub rating: ???,
    // Information about who is here now. If present, there is always a count, the number of people here. If viewing details and there is a logged-in user, there is also a groups field with friends and others as types.
    // pub hereNow: ???
    // Seconds since epoch when the venue was created.
    // pub createdAt: ???
    // A count and groups of photos for this venue. Group types are checkin and venue. Not all items will be present.
    // pub photos: ???,
    // Contains the total count of tips and groups with friends and others as groupTypes. Groups may change over time.
    // pub tips: ??,
    // ??
    #[serde(rename = "referralId")]
    pub referral_id: Option<String>,
    #[serde(rename = "hasPerk")]
    pub has_perk: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Venues {
    pub venues: Vec<Venue>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VenueWrapper {
    pub venue: Venue,
}


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

/// Entry point interface for interacting with Github API
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
    pub fn new<A>(version: A, credentials: Option<Credentials>, handle: &Handle) -> Self
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

    pub fn venue<I>(&self, id: I) -> Future<Response<VenueWrapper>>
    where
        I: Into<String>,
    {
        self.request(
            Method::Get,
            format!("{host}/v2/venues/{id}", host = self.host, id = id.into()),
            None,
        )
    }

    /// https://developer.foursquare.com/docs/api/venues/search
    pub fn search(&self) -> Future<Response<Venues>> {
        self.request(
            Method::Get,
            format!(
                "{host}/v2/venues/search?{query}",
                host = self.host,
                query = form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(vec![("ll", "40.7243,-74.0018"), ("query", "coffee")])
                    .finish()
            ),
            None,
        )
    }

    fn request<Out>(&self, method: Method, uri: String, body: Option<Vec<u8>>) -> Future<Out>
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