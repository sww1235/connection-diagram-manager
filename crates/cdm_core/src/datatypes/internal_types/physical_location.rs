use num_rational::Rational64;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PhysicalLocation {
    street_address: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip_code: Option<String>,
    latitude: Rational64,
    longitude: Rational64,
    structured_location_id: Option<String>,
    planet: Option<String>,
    building: Option<String>,
}
