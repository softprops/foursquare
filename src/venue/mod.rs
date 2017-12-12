//! Venue interfaces

use std::fmt;

use hyper::client::Connect;
use serde_urlencoded;

use {Client, Future, Response};

pub struct Venues<C>
where
    C: Connect + Clone,
{
    client: Client<C>,
}

impl<C: Connect + Clone> Venues<C> {
    #[doc(hidden)]
    pub(crate) fn new(client: Client<C>) -> Self {
        Self { client }
    }

    /// Get the tips for a single venue
    ///
    /// See the official
    /// [api docs](https://developer.foursquare.com/docs/api/venues/hours)
    /// for more information
    pub fn tips<I>(
        &self,
        id: I,
        options: &TipsOptions,
    ) -> Future<Response<TipsResponse>>
    where
        I: Into<String>,
    {
        self.client.get(format!(
            "{host}/v2/venues/{id}/tips?={query}",
            host = self.client.host,
            id = id.into(),
            query = serde_urlencoded::to_string(options).unwrap()
        ))
    }

    /// Get the hours for a single venue
    ///
    /// See the official
    /// [api docs](https://developer.foursquare.com/docs/api/venues/hours)
    /// for more information
    pub fn hours<I>(
        &self,
        id: I,
        options: &HoursOptions,
    ) -> Future<Response<VenueHoursResponse>>
    where
        I: Into<String>,
    {
        self.client.get(format!(
            "{host}/v2/venues/{id}/hours?={query}",
            host = self.client.host,
            id = id.into(),
            query = serde_urlencoded::to_string(options).unwrap()
        ))
    }

    /// Get the details for single venue
    ///
    /// See the official
    /// [api docs](https://developer.foursquare.com/docs/api/venues/details)
    /// for more information
    pub fn get<I>(
        &self,
        id: I,
        options: &VenueDetailsOptions,
    ) -> Future<Response<VenueResponse>>
    where
        I: Into<String>,
    {
        self.client.get(format!(
            "{host}/v2/venues/{id}?={query}",
            host = self.client.host,
            id = id.into(),
            query = serde_urlencoded::to_string(options).unwrap()
        ))
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
        self.client.get(format!(
            "{host}/v2/venues/search?{query}",
            host = self.client.host,
            query = serde_urlencoded::to_string(options).unwrap()
        ))
    }

    /// Type ahead suggestions
    ///
    /// See the official
    /// [api docs](https://developer.foursquare.com/docs/api/venues/suggestcompletion)
    /// for more information
    pub fn suggest(
        &self,
        options: &SuggestOptions,
    ) -> Future<Response<SuggestResponse>> {
        self.client.get(format!(
            "{host}/v2/venues/suggestcompletion?{query}",
            host = self.client.host,
            query = serde_urlencoded::to_string(options).unwrap()
        ))
    }

    /// Get venue recommendations in a target geography
    ///
    /// See the official
    /// [api docs](https://developer.foursquare.com/docs/api/venues/recommendations)
    /// for more information
    pub fn recommendations(
        &self,
        options: &RecommendationsOptions,
    ) -> Future<Response<RecommendationsResponse>> {
        self.client.get(format!(
            "{host}/v2/search/recommendations/?{query}",
            host = self.client.host,
            query = serde_urlencoded::to_string(options).unwrap()
        ))
    }

    /// Explore venues in a target geography
    ///
    /// See the official
    /// [api docs](https://developer.foursquare.com/docs/api/venues/explore)
    /// for more information
    pub fn explore(
        &self,
        options: &ExploreOptions,
    ) -> Future<Response<ExploreResponse>> {
        self.client.get(format!(
            "{host}/v2/venues/explore?{query}",
            host = self.client.host,
            query = serde_urlencoded::to_string(options).unwrap()
        ))
    }
}

// representations

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Intent {
    Checkin,
    Global,
    Browse,
    Match,
}

impl Default for Intent {
    fn default() -> Self {
        Intent::Checkin
    }
}

/// Search api options.
///
/// Use SearchOptions::builder() interface to construct these
#[derive(Default, Debug, Deserialize, Serialize, Builder)]
#[builder(setter(into), default)]
pub struct SearchOptions {
    /// required unless near is provided. Latitude and longitude of the user’s location. Optional if using intent=global
    #[serde(skip_serializing_if = "String::is_empty")]
    ll: String,
    /// required unless ll is provided. A string naming a place in the world. If the near string is not geocodable, returns a failed_geocode error. Otherwise, searches within the bounds of the geocode and adds a geocode object to the response.
    #[serde(skip_serializing_if = "String::is_empty")]
    near: String,
    /// One of the values below, indicating your intent in performing the search. If no value is specified, defaults to checkin.
    intent: Option<Intent>,
    /// Limit results to venues within this many meters of the specified location. Defaults to a city-wide area. Only valid for requests with intent=browse, or requests with intent=checkin and categoryId or query. Does not apply to intent=match requests. The maximum supported radius is currently 100,000 meters.
    radius: Option<u32>,
    /// With ne, limits results to the bounding box defined by the latitude and longitude given by sw as its south-west corner, and ne as its north-east corner. The bounding box is only supported for intent=browse searches. Not valid with ll or radius. Bounding boxes with an area up to approximately 10,000 square kilometers are supported.
    sw: Option<String>,
    /// See sw.
    ne: Option<String>,
    /// A search term to be applied against venue names.
    query: Option<String>,
    /// Number of results to return, up to 50.
    limit: Option<u32>,
    /// A comma separated list of categories to limit results to. If you specify categoryId. specifying a radius may improve results. If specifying a top-level category, all sub-categories will also match the query. Does not apply to intent=match requests.
    #[serde(rename = "categoryId")]
    category_id: Option<String>,
    /// Accuracy of latitude and longitude, in meters.
    #[serde(rename = "llAcc")]
    ll_acc: Option<f64>,
    /// Altitude of the user’s location, in meters.
    alt: Option<u32>,
    /// Accuracy of the user’s altitude, in meters.
    #[serde(rename = "altAcc")]
    alt_acc: Option<f64>,
    /// A third-party URL which we will attempt to match against our map of venues to URLs.
    url: Option<String>,
    /// Identifier for a known third party that is part of our map of venues to URLs, used in conjunction with linkedId.
    #[serde(rename = "providerId")]
    provider_id: Option<String>,
    /// Identifier used by third party specified in providerId, which we will attempt to match against our map of venues to URLs.
    #[serde(rename = "linkedId")]
    linked_id: Option<String>,
    /// [Internationalization](https://developer.foursquare.com/docs/api/configuration/internationalization)
    pub locale: Option<String>,
}

impl SearchOptions {
    pub fn builder() -> SearchOptionsBuilder {
        SearchOptionsBuilder::default()
    }
}

#[derive(Default, Debug, Deserialize, Serialize, Builder)]
#[builder(setter(into), default)]
pub struct SuggestOptions {
    /// required Latitude and longitude of the user’s location. (Required for query searches)
    #[serde(skip_serializing_if = "String::is_empty")]
    ll: String,
    /// required unless ll is provided. A string naming a place in the world. If the near string is not geocodable, returns a failed_geocode error. Otherwise, searches within the bounds of the geocode. Adds a geocode object to the response. (Required for query searches)
    #[serde(skip_serializing_if = "String::is_empty")]
    near: String,
    /// Limit results to venues within this many meters of the specified location. Defaults to a city-wide area. Only valid for requests with intent=browse, or requests with intent=checkin and categoryId or query. Does not apply to intent=match requests. The maximum supported radius is currently 100,000 meters.
    radius: Option<u32>,
    /// With ne, limits results to the bounding box defined by the latitude and longitude given by sw as its south-west corner, and ne as its north-east corner. The bounding box is only supported for intent=browse searches. Not valid with ll or radius. Bounding boxes with an area up to approximately 10,000 square kilometers are supported.
    sw: Option<String>,
    /// See sw.
    ne: Option<String>,
    /// A search term to be applied against venue names.
    query: Option<String>,
    /// Number of results to return, up to 50.
    limit: Option<u32>,
    /// Accuracy of latitude and longitude, in meters.
    #[serde(rename = "llAcc")]
    ll_acc: Option<f64>,
    /// Altitude of the user’s location, in meters.
    alt: Option<u32>,
    /// Accuracy of the user’s altitude, in meters.
    #[serde(rename = "altAcc")]
    alt_acc: Option<f64>,
    /// [Internationalization](https://developer.foursquare.com/docs/api/configuration/internationalization)
    pub locale: Option<String>,
}

impl SuggestOptions {
    pub fn builder() -> SuggestOptionsBuilder {
        SuggestOptionsBuilder::default()
    }
}

/// Venue tips api options.
///
/// Use TipsOptions::builder() interface to construct these
#[derive(Default, Debug, Deserialize, Serialize, Builder)]
#[builder(setter(into), default)]
pub struct TipsOptions {
    /// One of friends, recent, or popular.
    sort: Option<String>,
    /// Number of results to return, up to 500.
    limit: Option<u32>,
    /// Used to page through results.
    offset: Option<u32>,
    /// [Internationalization](https://developer.foursquare.com/docs/api/configuration/internationalization)
    locale: Option<String>,
}

impl TipsOptions {
    pub fn builder() -> TipsOptionsBuilder {
        TipsOptionsBuilder::default()
    }
}

/// Venue hours api options.
///
/// Use VenueDetailsOptions::builder() interface to construct these
#[derive(Default, Debug, Deserialize, Serialize, Builder)]
#[builder(setter(into), default)]
pub struct HoursOptions {
    /// [Internationalization](https://developer.foursquare.com/docs/api/configuration/internationalization)
    locale: Option<String>,
}

impl HoursOptions {
    pub fn builder() -> HoursOptionsBuilder {
        HoursOptionsBuilder::default()
    }
}

/// Venue details api options.
///
/// Use VenueDetailsOptions::builder() interface to construct these
#[derive(Default, Debug, Deserialize, Serialize, Builder)]
#[builder(setter(into), default)]
pub struct VenueDetailsOptions {
    /// [Internationalization](https://developer.foursquare.com/docs/api/configuration/internationalization)
    locale: Option<String>,
}

impl VenueDetailsOptions {
    pub fn builder() -> VenueDetailsOptionsBuilder {
        VenueDetailsOptionsBuilder::default()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Feature {
    #[serde(rename = "0")]
    TakesCreditCards,
    #[serde(rename = "1")]
    TakesReservations,
    #[serde(rename = "2")]
    OffersDelivery,
    #[serde(rename = "3")]
    OffersTakeOut,
    #[serde(rename = "4")]
    Wifi,
    #[serde(rename = "5")]
    OutdoorSeating,
    #[serde(rename = "7")]
    Liked,
    #[serde(rename = "8")]
    RecentlyOpened,
    #[serde(rename = "9")]
    NotChain,
    #[serde(rename = "10")]
    OnlineReservations,
    #[serde(rename = "13")]
    DogFriendly,
    #[serde(rename = "14")]
    ParkingLot,
    #[serde(rename = "15")]
    HappyHour,
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Feature::TakesCreditCards => "0",
                Feature::TakesReservations => "1",
                Feature::OffersDelivery => "2",
                Feature::OffersTakeOut => "3",
                Feature::Wifi => "4",
                Feature::OutdoorSeating => "5",
                Feature::Liked => "7",
                Feature::RecentlyOpened => "8",
                Feature::NotChain => "9",
                Feature::OnlineReservations => "10",
                Feature::DogFriendly => "13",
                Feature::ParkingLot => "14",
                Feature::HappyHour => "15",
            }
        )
    }
}

/// serialize features as a comma-delimited string
fn serialize_comma_delim<S, D>(
    x: &Option<Vec<D>>,
    ser: S,
) -> ::std::result::Result<S::Ok, S::Error>
where
    S: ::serde::Serializer,
    D: fmt::Display,
{
    match *x {
        Some(ref feats) => {
            ser.serialize_str(
                feats
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<_>>()
                    .join(",")
                    .as_ref(),
            )
        }
        _ => ser.serialize_none(),
    }
}


/// Recommedations api options.
///
/// Use RecommendationsOptions::builder() interface to construct these
#[derive(Default, Debug, Deserialize, Serialize, Builder)]
#[builder(setter(into), default)]
pub struct RecommendationsOptions {
    /// required unless near is provided. Latitude and longitude of the user’s location.
    #[serde(skip_serializing_if = "String::is_empty")]
    ll: String,
    /// required unless ll is provided. A string naming a place in the world. If the near string is not geocodable, returns a failed_geocode error. Otherwise, searches within the bounds of the geocode and adds a geocode object to the response.
    #[serde(skip_serializing_if = "String::is_empty")]
    near: String,
    /// Accuracy of latitude and longitude, in meters.
    #[serde(rename = "llAcc")]
    ll_acc: Option<f64>,
    /// Altitude of the user’s location, in meters.
    alt: Option<u32>,
    /// Accuracy of the user’s altitude, in meters.
    #[serde(rename = "altAcc")]
    alt_acc: Option<f64>,
    /// Radius to search within, in meters. If radius is not specified, a suggested radius will be used based on the density of venues in the area. The maximum supported radius is currently 100,000 meters.
    radius: Option<u32>,
    /// One of: food, breakfast, brunch, lunch, coffee, dinner, dessert, drinks, shopping, fun, sights. Specifies the top-level “intent” for a search.
    intent: Option<String>,
    /// One of: 1, 2, 3, 4. Only return venues that match the specified price(s), 1 being “$” and 4 being “”. Supports multiple values.
    #[serde(serialize_with = "serialize_comma_delim")]
    prices: Option<Vec<u16>>,
    /// A search term to be applied against venue names.
    query: Option<String>,
    /// Return values that match the specified categories, after the `query` parameter is applied.
    #[serde(serialize_with = "serialize_comma_delim")]
    categories: Option<Vec<String>>,
    /// Return values that match the specified categories when there is no `query` parameter provided.
    #[serde(rename = "categoryId")]
    category_id: Option<String>,
    /// Number of results to return, up to 50.
    limit: Option<u32>,
    /// Used to page through results, up to 50.
    offset: Option<u32>,
    /// Specifies what features (takes credit cards, offers delivery, etc.) that the returned venues should have. The following param values correspond to various features this endpoint supports.
    /// 0 Takes credit cards.
    /// 1 Takes reservations.
    /// 2 Offers delivery.
    /// 3 Offers take out.
    /// 4 Has Wi-Fi.
    /// 5 Has outdoor seating.
    /// 7 User has liked this venue.
    /// 8 Recently opened.
    /// 9 Not part of a chain.
    /// 10 Takes online reservations.
    /// 13 Dog-friendly.
    /// 14 Has parking.
    /// 15 Has a happy hour.
    #[serde(serialize_with = "serialize_comma_delim")]
    features: Option<Vec<Feature>>,
    /// Boolean flag to only include venues that are open now. This prefers official provider hours but falls back to popular check-in hours.
    #[serde(rename = "openNow")]
    open_now: Option<bool>,
    /// Boolean flag to sort the results by distance instead of relevance.
    #[serde(rename = "sortByDistance")]
    sort_by_distance: Option<bool>,
    /// If you make an authenticated request to this endpoint, you can make results more personal based on the user’s experiences on Swarm and Foursquare (e.g., only return venues that the user has saved to a list or ones that the user has liked before). The following table documents the currently-available personalizations.
    personalization: Option<String>,
    /// 1–7 for Monday–Sunday. Only return results that are open on this day.
    #[serde(rename = "localDay")]
    local_day: Option<String>,
    /// Only return results that are open at this time. HH in 24-hr format.
    #[serde(rename = "localTime")]
    local_time: Option<String>,
    /// [Internationalization](https://developer.foursquare.com/docs/api/configuration/internationalization)
    locale: Option<String>,
}

impl RecommendationsOptions {
    pub fn builder() -> RecommendationsOptionsBuilder {
        RecommendationsOptionsBuilder::default()
    }
}

/// Explore api options.
///
/// Use ExploreOptions::builder() interface to construct these
#[derive(Default, Debug, Deserialize, Serialize, Builder)]
#[builder(setter(into), default)]
pub struct ExploreOptions {
    /// required unless near is provided. Latitude and longitude of the user’s location.
    #[serde(skip_serializing_if = "String::is_empty")]
    ll: String,
    /// required unless ll is provided. A string naming a place in the world. If the near string is not geocodable, returns a failed_geocode error. Otherwise, searches within the bounds of the geocode and adds a geocode object to the response.
    #[serde(skip_serializing_if = "String::is_empty")]
    near: String,
    /// Accuracy of latitude and longitude, in meters.
    #[serde(rename = "llAcc")]
    ll_acc: Option<f64>,
    /// Altitude of the user’s location, in meters.
    alt: Option<u32>,
    /// Accuracy of the user’s altitude, in meters.
    #[serde(rename = "altAcc")]
    alt_acc: Option<f64>,
    /// Radius to search within, in meters. If radius is not specified, a suggested radius will be used based on the density of venues in the area. The maximum supported radius is currently 100,000 meters.
    radius: Option<u32>,
    /// One of food, drinks, coffee, shops, arts, outdoors, sights, trending, nextVenues (venues frequently visited after a given venue), or topPicks (a mix of recommendations generated without a query from the user). Choosing one of these limits results to venues with the specified category or property.
    section: Option<String>,
    /// A term to be searched against a venue’s tips, category, etc. The query parameter has no effect when a section is specified.
    query: Option<String>,
    /// Number of results to return, up to 50.
    limit: Option<u32>,
    /// Used to page through results, up to 50.
    offset: Option<u32>,
    /// Pass new or old to limit results to places the acting user hasn’t been or has been, respectively. Omitting this parameter returns a mixture of old and new venues.
    novelty: Option<String>,
    /// Pass visited or notvisited to limit results to places the acting user’s friends have or haven’t been, respectively. Omitting this parameter returns a mixture of venues to which the user’s friends have or haven’t been.
    #[serde(rename = "friendVisits")]
    friend_visits: Option<String>,
    /// Pass any to retrieve results for any time of day. Omitting this parameter returns results targeted to the current time of day.
    time: Option<String>,
    /// Pass any to retrieve results for any day of the week. Omitting this parameter returns results targeted to the current day of the week.
    day: Option<String>,
    /// Boolean flag to include a photo in the response for each venue, if one is available. Default is 0 (no photos). Photos are returned as part of the venue JSON object.
    #[serde(rename = "venuePhotos")]
    venue_photos: Option<u16>, // 1 or 0
    /// A venue ID to use in combination with the intent=nextVenues parameter, which returns venues users often visit after a given venue. If intent=nextVenues is specified but lastVenue is not, the user’s last check-in will be used if it is within 2 hours. If the user has not checked in within the last 2 hours, no results will be returned.
    #[serde(rename = "lastVenue")]
    last_venue: Option<String>,
    /// Boolean flag to only include venues that are open now. This prefers official provider hours but falls back to popular check-in hours.
    #[serde(rename = "openNow")]
    open_now: Option<u16>, // 1 or 0
    /// Boolean flag to sort the results by distance instead of relevance.
    #[serde(rename = "sortByDistance")]
    sort_by_distance: Option<u16>, // 1 or 0,
    /// Comma separated list of price points. Currently the valid range of price points are [1,2,3,4], 1 being the least expensive, 4 being the most expensive. For food venues, in the United States, 1 is < $10 an entree, 2 is $10-$20 an entree, 3 is $20-$30 an entree, 4 is > $30 an entree.
    price: Option<String>,
    /// Boolean flag to only include venues that the user has saved on their To-Do list or to another list.
    saved: Option<u16>, // 1 or 0
    /// [Internationalization](https://developer.foursquare.com/docs/api/configuration/internationalization)
    locale: Option<String>,
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
    pub facebook: Option<String>,
    #[serde(rename = "facebookUsername")]
    pub facebook_username: Option<String>,
    #[serde(rename = "facebookName")]
    pub facebook_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Price {
    pub tier: u16,
    pub message: String,
    pub currency: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub address: Option<String>,
    #[serde(rename = "crossStreet")]
    pub cross_street: Option<String>,
    pub lat: f64,
    pub lng: f64,
    pub distance: Option<u32>,
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    /// Returns  None for suggest requests
    pub cc: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: String,
    /// Returns None for suggest requests
    #[serde(rename = "formattedAddress")]
    pub formatted_address: Option<Vec<String>>,
}

/// Icon photo
///
/// Pieces needed to construct category icons at various sizes. Combine prefix with a size (32, 44, 64, and 88 are available) and suffix, e.g. https://foursquare.com/img/categories/food/default_64.png. To get an image with a gray background, use bg_ before the size, e.g. https://foursquare.com/img/categories_v2/food/icecream_bg_32.png.
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
pub struct Photos {
    pub count: u64,
    pub groups: Vec<Group<PhotoItem>>,
}

/// venue photo
///
/// see [this doc](https://developer.foursquare.com/docs/api/photos/details)
/// for photo url construction
#[derive(Debug, Deserialize, Serialize)]
pub struct PhotoItem {
    pub id: String,
    pub prefix: String,
    pub suffix: String,
    pub width: u16,
    pub height: u16,
    pub user: Option<User>,
    pub visibility: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub photo: UserPhoto,
}

/// user photo
///
/// see [this doc](https://developer.foursquare.com/docs/api/photos/details)
/// for photo url construction
#[derive(Debug, Deserialize, Serialize)]
pub struct UserPhoto {
    pub prefix: String,
    pub suffix: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeWindow {
    pub start: String,
    pub end: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Timeframe {
    pub days: Vec<u16>,
    #[serde(rename = "includesToday")]
    pub includes_today: Option<bool>,
    pub open: Vec<TimeWindow>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VenueHours {
    pub timeframes: Vec<Timeframe>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VenueHoursResponse {
    /// An array of timeframes of open hours.
    pub hours: VenueHours,
    /// An array of timeframes of popular hours.
    pub popular: VenueHours,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Hours {
    pub status: Option<String>,
    #[serde(rename = "isOpen")]
    pub is_open: bool,
    #[serde(rename = "isLocalHoliday")]
    pub is_local_holiday: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tip {
    pub id: String,
    pub text: String,
    #[serde(rename = "canonicalUrl")]
    pub canonical_url: String,
    pub photo: Option<PhotoItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tips {
    pub count: u32,
    pub items: Vec<Tip>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TipsResponse {
    pub tips: Tips,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Venue {
    /// A unique string identifier for this venue.
    pub id: String,
    /// The best known name for this venue.
    pub name: String,
    /// An object containing none, some, or all of twitter, phone, and formattedPhone. All are strings.
    /// Will be None for suggest requests
    pub contact: Option<Contact>,
    /// An object containing none, some, or all of address (street address), crossStreet, city, state, postalCode, country, lat, lng, and distance. All fields are strings, except for lat, lng, and distance. Distance is measured in meters. Some venues have their locations intentionally hidden for privacy reasons (such as private residences). If this is the case, the parameter isFuzzed will be set to true, and the lat/lng parameters will have reduced precision.
    pub location: Location,
    /// An array, possibly empty, of categories that have been applied to this venue. One of the categories will have a primary field indicating that it is the primary category for the venue. For the complete category tree, see categories.
    pub categories: Vec<Category>,
    /// Boolean indicating whether the owner of this business has claimed it and verified the information.
    /// Will be be None for suggest requests
    pub verified: Option<bool>,
    // Contains checkinsCount (total checkins ever here), usersCount (total users who have ever checked in here), and tipCount (number of tips here).
    // pub stats: Stats
    /// URL of the venue’s website, typically provided by the venue manager.
    pub url: Option<String>,
    /// Contains the hours during the week that the venue is open along with any named hours segments in a human-readable format. For machine readable hours see venues/hours
    pub hours: Option<Hours>,
    /// Contains the hours during the week when people usually go to the venue. For machine readable hours see venues/hours.
    #[serde(rename = "hasMenu")]
    pub has_menu: Option<bool>,
    /// An object containing url and mobileUrl that display the menu information for this venue.
    pub menu: Option<Menu>,
    /// An object containing the price tier from 1 (least pricey) - 4 (most pricey) and a message describing the price tier.
    pub price: Option<Price>,
    // Information about who is here now. If present, there is always a count, the number of people here. If viewing details and there is a logged-in user, there is also a groups field with friends and others as types.
    // pub hereNow: ???
    // Seconds since epoch when the venue was created.
    // pub createdAt: ???
    /// A count and groups of photos for this venue. Group types are checkin and venue. Not all items will be present.
    /// Will typically not be present for search requests
    pub photos: Option<Photos>,
    // Contains the total count of tips and groups with friends and others as groupTypes. Groups may change over time.
    // pub tips: ??,
    // ??
    #[serde(rename = "referralId")]
    pub referral_id: Option<String>,
    #[serde(rename = "hasPerk")]
    pub has_perk: Option<bool>,
    /// Numerical rating of the venue (0 through 10). Not all venues will have a rating.
    pub rating: Option<f32>,
    #[serde(rename = "ratingSignals")]
    pub rating_signals: Option<u64>,
    /// time zone, only present in details requests
    #[serde(rename = "timeZone")]
    pub time_zone: Option<String>,
    /// Attributes associated with the venue, such as price tier, whether the venue takes reservations, and parking availability.
    /// only present in details requests
    pub attributes: Option<AttributeGroups>,
    /// The canonical URL for this venue, e.g. https://foursquare.com/v/foursquare-hq/4ab7e57cf964a5205f7b20e3
    /// only present in details requests
    #[serde(rename = "canonicalUrl")]
    pub canonical_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SuggestResponse {
    pub minivenues: Vec<Venue>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResponse {
    pub venues: Vec<Venue>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VenueItem {
    pub venue: Venue,
    #[serde(rename = "referralId")]
    pub referral_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Group<I> {
    pub name: String,
    #[serde(rename = "type")]
    pub group_type: String,
    pub count: Option<u64>,
    pub items: Vec<I>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct AttributeGroups {
    pub groups: Vec<Group<AttributeItem>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AttributeItem {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "displayValue")]
    pub display_value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Warning {
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExploreResponse {
    pub warning: Option<Warning>,
    /// If no radius was specified in the request, presents the radius that was used for the query (based upon the density of venues in the query area).
    #[serde(rename = "suggestedRadius")]
    pub suggested_radius: Option<u32>,
    /// A text name for the location the user searched, e.g. “SoHo”.
    #[serde(rename = "headerLocation")]
    pub header_location: String,
    /// A full text name for the location the user searched, e.g. “SoHo, New York”.
    #[serde(rename = "headerFullLocation")]
    pub header_full_location: String,
    #[serde(rename = "headerLocationGranularity")]
    pub header_location_granularity: String,
    pub query: Option<String>,
    #[serde(rename = "totalResults")]
    pub total_results: u64,
    /// An array of objects representing groups of recommendations. Each group contains a type such as “recommended” a human-readable (eventually localized) name such as “Recommended Places,” and an array items of recommendation objects, which have an ordered list of objects which contain reasons and venue. The reasons are count and items, where each item has a type such as “social” and a message about why this place may be of interest to the acting user. The venues are compact venues that include stats and hereNow data. We encourage clients to be robust against the introduction or removal of group types by treating the groups as opaque objects to be displayed or by placing unfamiliar groups in a catchall group.
    pub groups: Vec<Group<VenueItem>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecommendationsResponse {
    #[serde(rename = "normalizedQuery")]
    pub normalized_query: Option<String>,
    pub group: RecommendationsGroup,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecommendationsGroup {
    #[serde(rename = "totalResults")]
    pub total_results: u64,
    pub results: Vec<Recommendation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Recommendation {
    #[serde(rename = "displayType")]
    pub display_type: String,
    pub venue: Venue,
    pub photo: Option<PhotoItem>,
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

    #[test]
    fn default_intent() {
        let default: Intent = Default::default();
        assert_eq!(default, Intent::Checkin)
    }
}