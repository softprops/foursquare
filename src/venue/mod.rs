//! Venue interfaces

use hyper::Method;
use hyper::client::Connect;
use serde_urlencoded;
use url::form_urlencoded;

use {Client, Future, Response};

pub struct Venues<C>
where
    C: Connect + Clone,
{
    client: Client<C>,
}

impl<C: Connect + Clone> Venues<C> {
    pub fn new(client: Client<C>) -> Self {
        Self { client }
    }

    /// Get the defaults for single venue
    ///
    /// See the official
    /// [api docs](https://developer.foursquare.com/docs/api/venues/details)
    /// for more information
    pub fn get<I>(&self, id: I) -> Future<Response<VenueResponse>>
    where
        I: Into<String>,
    {
        self.client.request(
            Method::Get,
            format!(
                "{host}/v2/venues/{id}",
                host = self.client.host,
                id = id.into()
            ),
            None,
        )
    }

    /// Search for venues
    ///
    /// See the official
    /// [api docs](https://developer.foursquare.com/docs/api/venues/search)
    /// for more information
    pub fn search(
        &self,
        options: &SearchOptions,
    ) -> Future<Response<SearchResponse>> {
        self.client.request(
            Method::Get,
            format!(
                "{host}/v2/venues/search?{query}",
                host = self.client.host,
                query = serde_urlencoded::to_string(options).unwrap()
                /*query = form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(
                        vec![("ll", "37.5665,126.9780"), ("query", "coffee")],
                    )
                    .finish()*/
            ),
            None,
        )
    }

    /// Get recommendations on venues
    ///
    /// See the official
    /// [api docs](https://developer.foursquare.com/docs/api/venues/explore)
    /// for more information
    pub fn explore(
        &self,
        options: &ExploreOptions,
    ) -> Future<Response<ExploreResponse>> {
        self.client.request(
            Method::Get,
            format!(
                "{host}/v2/venues/explore?{query}",
                host = self.client.host,
                query = form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(
                        vec![("ll", "37.5665,126.9780"), ("query", "coffee")],
                    )
                    .finish()
            ),
            None,
        )
    }
}

// representations

/// Search api options.
///
/// Use SearchOptions::builder() interface to construct these
#[derive(Default, Debug, Deserialize, Serialize, Builder)]
#[builder(setter(into), default)]
pub struct SearchOptions {
    #[serde(skip_serializing_if = "String::is_empty")]
    ll: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    near: String,
    intent: Option<String>,
    radius: Option<u32>,
    sw: Option<String>,
    ne: Option<String>,
    query: Option<String>,
    /// Number of results to return, up to 50.
    limit: Option<u32>,
    #[serde(rename = "categoryId")]
    category_id: Option<String>,
    llAcc: Option<f64>,
    alt: Option<u32>,
    altAcc: Option<f64>,
    url: Option<String>,
    #[serde(rename = "providerId")]
    provider_id: Option<String>,
    #[serde(rename = "linkedId")]
    linked_id: Option<String>,
}

impl SearchOptions {
    pub fn builder() -> SearchOptionsBuilder {
        SearchOptionsBuilder::default()
    }
}

/// Explore api options.
///
/// Use ExploreOptions::builder() interface to construct these
#[derive(Default, Debug, Deserialize, Serialize, Builder)]
#[builder(setter(into), default)]
pub struct ExploreOptions {
    #[serde(skip_serializing_if = "String::is_empty")]
    ll: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    near: String,
    intent: Option<String>,
    radius: Option<u32>,
    section: Option<String>,
    query: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
    novelty: Option<String>,
    #[serde(rename = "friendVisits")]
    friend_visits: Option<String>,
    time: Option<String>,
    day: Option<String>,
    #[serde(rename = "venuePhotos")]
    venue_photos: Option<u16>, // 1 or 0
    #[serde(rename = "lastVenue")]
    last_venue: Option<String>,
    #[serde(rename = "openNow")]
    open_now: Option<u16>, // 1 or 0
    #[serde(rename = "sortByDistance")]
    sort_by_distance: Option<u16>, // 1 or 0,
    price: Option<String>,
    saved: Option<u16>, // 1 or 0
}

impl ExploreOptions {
    pub fn builder() -> ExploreOptionsBuilder {
        ExploreOptionsBuilder::default()
    }
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
    pub city: Option<String>,
    pub state: Option<String>,
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

    /// Contains the hours during the week that the venue is open along with any named hours segments in a human-readable format. For machine readable hours see venues/hours
    /// pub hours: Option<Hours>,
    /// Contains the hours during the week when people usually go to the venue. For machine readable hours see venues/hours.
    /// pub popular: Hours
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
pub struct SearchResponse {
    pub venues: Vec<Venue>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub venue: Venue,
    #[serde(rename = "referralId")]
    pub referral_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    pub name: String,
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExploreResponse {
    /// If no radius was specified in the request, presents the radius that was used for the query (based upon the density of venues in the query area).
    #[serde(rename = "suggestedRadius")]
    pub suggested_radius: u32,
    /// A text name for the location the user searched, e.g. “SoHo”.
    #[serde(rename = "headerLocation")]
    pub header_location: String,
    /// A full text name for the location the user searched, e.g. “SoHo, New York”.
    #[serde(rename = "headerFullLocation")]
    pub header_full_location: String,
    #[serde(rename = "headerLocationGranularity")]
    pub header_location_granularity: String,
    pub query: String,
    #[serde(rename = "totalResults")]
    pub total_results: u64,
    /// An array of objects representing groups of recommendations. Each group contains a type such as “recommended” a human-readable (eventually localized) name such as “Recommended Places,” and an array items of recommendation objects, which have an ordered list of objects which contain reasons and venue. The reasons are count and items, where each item has a type such as “social” and a message about why this place may be of interest to the acting user. The venues are compact venues that include stats and hereNow data. We encourage clients to be robust against the introduction or removal of group types by treating the groups as opaque objects to be displayed or by placing unfamiliar groups in a catchall group.
    pub groups: Vec<Group>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VenueResponse {
    pub venue: Venue,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_options_serialize() {
        assert_eq!(
            serde_urlencoded::to_string(
                &SearchOptions::builder().near("foo bar").build().unwrap(),
            ).unwrap(),
            "near=foo+bar"
        )
    }
}