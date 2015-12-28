use std::str::FromStr;
use std::fmt;

use Utm;
use Accuracy;
use gzd::{Gzd, GridSquareId100k};
use LatLon;
use parser::MgrsParser;
use Lat;
use row::RowLetter;
use col::ColLetter;
use band::LatBand;
use datum::Datum;

fn get_accuracy(e: usize, n: usize) -> Option<Accuracy> {
    /*!
    Converts a number to grid reference, then calculates significant digits

    # Examples

    ```
    // in MGRS: 00001 02500
    assert_eq!(Accuracy::One, get_accuracy(1, 2500));
    // in MGRS: 00025 00250
    assert_eq!(Accuracy::One, get_accuracy(25, 250));
    // in MGRS: 00050 00500
    assert_eq!(Accuracy::Ten, get_accuracy(50, 500));
    // in MGRS: 00200 01000
    assert_eq!(Accuracy::OneHundred, get_accuracy(200, 1000));
    ```
    */
    let e_s = format!("{0:0>5}", e);
    let n_s = format!("{0:0>5}", n);
    let e_st = e_s.trim_right_matches('0');
    let n_st = n_s.trim_right_matches('0');
    if e_st.len() >= n_st.len() {
        Accuracy::from_num_digits(e_st.len())
    } else {
        Accuracy::from_num_digits(n_st.len())
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Mgrs {
    pub gzd: Gzd,
    pub gsid_100k: GridSquareId100k,
    pub easting: usize,
    pub northing: usize,
    pub accuracy: Accuracy,
}

impl Mgrs {
    fn new<L, R, C, D>(zone: u8, band: L, e100k: R, n100k: C, easting: usize, northing: usize, datum: D) -> Self
        where L: Into<LatBand>,
              R: Into<RowLetter>,
              C: Into<ColLetter>,
              D: Into<Datum> {
        /*!
        Creates an Mgrs grid reference object.

        ### Params

         * **zone**: 6° longitudinal zone (1..60 covering 180°W..180°E).
         * **band**: 8° latitudinal band (C..X covering 80°S..84°N).
         * **e100k**: First letter (E) of 100km grid square.
         * **n100k**: Second letter (N) of 100km grid square.
         * **easting**: Easting in metres within 100km grid square (without leading `0`s).
         * **northing**: Northing in metres within 100km grid square (without leading `0`s).
         * **datum**: Datum UTM coordinate is based on.

        # Panics

        If invalid MGRS grid reference northing or easting (such as either greater than 5 digits)

        # Examples

        ```
        let mgrs = Mgrs::new(31, 'U', 'D', 'Q', 48251, 11932);
        assert_eq!("31U DQ 48251 11932", &*mgrs.to_string());
        ```
        */

        Mgrs {
            gzd: Gzd { zone: zone, band: band },
            gsid_100k: GridSquareId100k{ col: e100k, row: n100k },
            easting: easting,
            northing: northing,
            accuracy: self::get_accuracy(easting, northing).expect("Invalid MGRS grid")
        }
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

    fn as_string(&self, accuracy: Accuracy) -> String {
        /*!
        Returns a string representation of an MGRS grid reference.

        To distinguish from civilian UTM coordinate representations, no space is included within
        the zone/band grid zone designator. Single digit zones are padded with a leading `0`.

        Components are separated by spaces: for a military-style unseparated string, use
        `mgrs.as_string(Accuracy::One).replace(" ", "");`

        ### Params
         * **accuracy** Precision of returned grid reference (eg `One` = 1m or 10 digit grid,
         `Ten` = 10m or 8 digit grid, etc.).

        ### Returns
         * This grid reference in standard format.

        # Examples

        ```
        let mgrs_str = "31U DQ 48251 11932";
        let mgrs = Mgrs::from(mgrs_str);
        assert_eq!(mgrs_str, &*mgrs.as_string());
        ```
        */

        let digits = accuracy.as_num_digits() / 2;
        // set required precision
        let easting = (f64::floor(self.easting / f64::powi(10, 5 - digits))) as usize;
        let northing = (f64::floor(self.northing / f64::powi(10, 5 - digits))) as usize;

        format!("{0:02}{1} {2}{3} {4:0<6$} {5:0<6$}", self.gzd.zone, self.gzd.band, self.gsid_100k.col, self.gsid_100k.row, easting, northing, digits)
    }
}

impl From<Utm> for Mgrs {
    fn from(utm: Utm) -> Self {
        utm.to_mgrs(None)
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

impl fmt::Display for Mgrs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.as_string(Accuracy::One))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Accuracy;

    #[test]
    fn getting_accuracy() {
        assert_eq!(mgrs::get_accuracy(25, 250), Accuracy::One);
        assert_eq!(mgrs::get_accuracy(5, 25), Accuracy::One);
        assert_eq!(mgrs::get_accuracy(256, 823), Accuracy::OneHundred);
        assert_eq!(mgrs::get_accuracy(12345, 354), Accuracy::One);
        assert_eq!(mgrs::get_accuracy(12000, 123), Accuracy::OneHundred);
        assert_eq!(mgrs::get_accuracy(1200, 1000), Accuracy::OneThousand);
        assert_eq!(mgrs::get_accuracy(10000, 1), Accuracy::One);
        assert_eq!(mgrs::get_accuracy(10000, 20000), Accuracy::TenThousand);
        assert_eq!(mgrs::get_accuracy(100, 1234), Accuracy::Ten);
    }
}
