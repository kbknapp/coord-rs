use std::str::FromStr;

use Utm;
use Accuracy;
use gzd::GridSquareId100k;
use LatLon;
use parser::MgrsParser;

#[derive(Default, Copy, Clone, Debug)]
pub struct Mgrs {
    pub utm: Utm,
    pub gsid_100k: GridSquareId100k,
    pub accuracy: Accuracy,
}

impl Mgrs {
    pub fn to_ll(self) -> [LatLon; 2] {
        /*!
        Conversion of MGRS to lat/lon.

        ### Params
         * **mgrs** Generic object that supports becoming an `Mgrs` struct.
        ### Return
         * An array of `latLon` structs which represents bottom-left, and top-right values in WGS84,
           representing the bounding box for the provided MGRS reference.
        */
        LatLon::from_mgrs(self).expect("failed to convert MGRS to Lat/Lon")
    }

    pub fn as_ll(&self) -> [LatLon; 2] {
        /*!
        Conversion of MGRS to lat/lon.

        ### Params
         * **mgrs** Generic object that supports becoming an `Mgrs` struct.
        ### Return
         * An array of `latLon` structs which represents bottom-left, and top-right values in WGS84,
           representing the bounding box for the provided MGRS reference.
        */
        LatLon::from_mgrs(self).expect("failed to convert MGRS to Lat/Lon")
    }

    /// Derives the centerpoint of an MGRS reference
    pub fn to_point(self) -> LatLon {
        LatLon::from(self.utm)
    }

    /// Derives the centerpoint of an MGRS reference
    pub fn as_point(&self) -> LatLon {
        LatLon::from(self.utm)
    }
}

impl From<Utm> for Mgrs {
    fn from(utm: Utm) -> Self {
        Mgrs {
            gsid_100k: utm.get_100k_id(),
            utm: utm,
            accuracy: Accuracy::One,
        }
    }
}

impl From<LatLon> for Mgrs {
    fn from(ll: LatLon) -> Self {
        ll.as_mgrs(None)
    }
}

impl<'a> From<&'a Mgrs> for Mgrs {
    fn from(m: &'a Mgrs) -> Self {
        m.clone()
    }
}

impl FromStr for Mgrs {
    type Err = ();
    /// Decode the UTM parameters from a MGRS string.
    /// @param {string} mgrs an UPPERCASE coordinate string is expected.
    /// @return {object} An object literal with easting, northing, zoneLetter,
    ///     zone_num and accuracy (in meters) properties.
    fn from_str(mgrs: &str) -> Result<Self, Self::Err> {
        Ok(MgrsParser::new(mgrs.as_bytes()).parse())
    }
}
