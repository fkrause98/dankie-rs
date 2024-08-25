use crate::types::Location;
use serde::Deserialize;

/// Represents a [`Venue`].
///
/// [`Venue`]: https://core.telegram.org/bots/api#venue
#[derive(Debug, PartialEq, Clone, Deserialize)]
#[non_exhaustive]
pub struct Venue {
    /// The location of the venue.
    pub location: Location,
    /// The title of the venue.
    pub title: String,
    /// The address of the venue.
    pub address: String,
    /// The foursquare ID of the venue.
    pub foursquare_id: Option<String>,
    /// The foursquare type of the venue.
    pub foursquare_type: Option<String>,
    /// The Google Places ID of the venue.
    pub google_place_id: Option<String>,
    /// The Google Places type of the venue.
    pub google_place_type: Option<String>,
}
