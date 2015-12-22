use std::str::FromStr;

use Utm;
use Accuracy;
use gzd::GridSquareId100k;
use LatLon;
use parser::MgrsParser;

#[derive(Default, Copy, Clone, Debug)]
pub struct Mgrs {
    pub gzd: Gzd,
    pub gsid_100k: GridSquareId100k,
    pub accuracy: Accuracy,
}

impl Mgrs {
    fn new(zone, band, e100k, n100k, easting, northing, datum) {
        /*!
        Creates an Mgrs grid reference object.

        @classdesc Convert MGRS grid references to/from UTM coordinates.

        @constructor
        @param  {number} zone - 6° longitudinal zone (1..60 covering 180°W..180°E).
        @param  {string} band - 8° latitudinal band (C..X covering 80°S..84°N).
        @param  {string} e100k - First letter (E) of 100km grid square.
        @param  {string} n100k - Second letter (N) of 100km grid square.
        @param  {number} easting - Easting in metres within 100km grid square.
        @param  {number} northing - Northing in metres within 100km grid square.
        @param  {LatLon.datum} [datum=WGS84] - Datum UTM coordinate is based on.
        @throws {Error} Invalid MGRS grid reference

        @example
          var mgrsRef = new Mgrs(31, 'U', 'D', 'Q', 48251, 11932); // 31U DQ 48251 11932
        */

        if (band.length != 1) throw new Error('Invalid MGRS grid reference');
        if (Mgrs.latBands.indexOf(band) == -1) throw new Error('Invalid MGRS grid reference');
        if (e100k.length!=1 || n100k.length!=1) throw new Error('Invalid MGRS grid reference');

        this.zone = Number(zone);
        this.band = band;
        this.e100k = e100k;
        this.n100k = n100k;
        this.easting = Number(easting);
        this.northing = Number(northing);
    }

    pub fn to_ll_rect(self) -> [LatLon; 2] {
        /*!
        Conversion of MGRS to lat/lon.

        ### Params
         * **mgrs** Generic object that supports becoming an `Mgrs` struct.
        ### Return
         * An array of `latLon` structs which represents bottom-left, and top-right values in WGS84,
           representing the bounding box for the provided MGRS reference.
        */
        LatLon::rect_from_mgrs(self).expect("failed to convert MGRS to Lat/Lon")
    }

    pub fn as_ll_rect(&self) -> [LatLon; 2] {
        /*!
        Conversion of MGRS to lat/lon.

        ### Params
         * **mgrs** Generic object that supports becoming an `Mgrs` struct.
        ### Return
         * An array of `latLon` structs which represents bottom-left, and top-right values in WGS84,
           representing the bounding box for the provided MGRS reference.
        */
        LatLon::rect_from_mgrs(self).expect("failed to convert MGRS to Lat/Lon")
    }

    /// Derives the centerpoint of an MGRS reference
    pub fn to_ll(self) -> LatLon {
        LatLon::from(self.utm)
    }

    /// Derives the centerpoint of an MGRS reference
    pub fn as_ll(&self) -> LatLon {
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
