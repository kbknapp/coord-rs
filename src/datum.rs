use std::str::FromStr;
use std::ascii::AsciiExt;

use Errors;
// major (equatorial) radius in meters
const WGS84_ELLIPSOID_A: f64 = 6378137.0;
// polar semi-minor axis in meters
const WGS84_ELLIPSOID_B: f64 = 6356752.314245;
// flattening
const WGS84_ELLIPSOID_F: f64 = 1.0 / 298.257223563;

#[derive(Copy, Clone, Debug)]
pub enum Datum {
    Wgs84
}

impl Datum {
    fn a(&self) -> f64 {
        match *self {
            Datum::Wgs84 => WGS84_ELLIPSOID_A
        }
    }
    fn b(&self) -> f64 {
        match *self {
            Datum::Wgs84 => WGS84_ELLIPSOID_B
        }
    }
    fn f(&self) -> f64 {
        match *self {
            Datum::Wgs84 => WGS84_ELLIPSOID_F
        }
    }
}

impl Default for Datum {
    fn default() -> Self {
        Datum::Wgs84
    }
}

///////////////////////////////////
///////////// impls ///////////////
///////////////////////////////////

impl<S: AsRef<str>> From<S> for Datum {
    fn from(s: S) -> Self {
        Datum::from(s.as_ref())
    }
}

impl FromStr for Datum {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let d = s.to_ascii_uppercase();
        match &*d {
            "WGS84" => Ok(Datum::Wgs84),
            _ => Err(Errors::InvalidDatum(s))
        }
    }
}
