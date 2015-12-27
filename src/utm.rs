use std::fmt;

use gzd::{Gzd, GridSquareId100k};
use get_100k_set_for_zone;
use latlon::LatLon;
use Mgrs;
use Accuracy;
use band::LatBand;
use datum::Datum;
use hemisphere::Hemisphere;
use col::ColLetter;
use row::RowLetter;

#[derive(Default, Copy, Clone, Debug)]
pub struct Utm {
    /// UTM 6° longitudinal zone (1..60 inclusive covering 180°W..180°E).
    pub zone: u8,
    /// N for northern hemisphere, S for southern hemisphere.
    pub hemisphere: Hemisphere,
    /// Easting in metres from false easting (-500km from central meridian).
    pub easting: i32,
    /// Northing in metres from equator (N) or from false northing -10,000km (S).
    pub northing: i32,
    /// Datum UTM coordinate is based on.
    pub datum: Datum,
    /// Meridian convergence (bearing of grid north clockwise from true north), in degrees
    pub convergence: Option<f64>,
    /// Grid scale factor
    pub scale: Option<f64>,

}

impl Utm {
    fn new<H, D>(zone: u8, hemisphere: H, easting: i32, northing: i32) -> Self
        where H: Into<Hemisphere>,
              D: Into<Datum> {
        /*!
        Creates a `Utm` coordinate struct.

        ### Params
            * **zone**: UTM 6° longitudinal zone (1..60 covering 180°W..180°E).
            * **hemisphere**: N for northern hemisphere, S for southern hemisphere.
            * **easting**: Easting in metres from false easting (-500km from central meridian).
            * **northing**: Northing in metres from equator (N) or from false northing -10,000km (S).

        # Examples

        ```
        let utm_coord = Utm::new(31, 'N', 448251, 5411932);
        ```

        # Panics

        This function will panic if an invalid zone number is passed as the `zone` param. valid
        values are 1..60 inclusive
        */

        if !(1<=zone && zone<=60) { panic!("Invalid UTM zone {}", zone); }
        // range-check easting/northing (with 40km overlap between zones) - this this worthwhile?
        //if (!(120e3<=easting && easting<=880e3)) throw new Error('Invalid UTM easting '+ easting);
        //if (!(0<=northing && northing<=10000e3)) throw new Error('Invalid UTM northing '+ northing);

        Utm {
            zone: zone,
            hemisphere: hemisphere.into(),
            easting: easting,
            northing: northing,
            datum: Datum::Wgs84,
            convergence: None,
            scale: None,
        }
    }

    pub fn from_ll(ll: &LatLon) -> Self {
        /*!
        Converts a set of lonitude and latitude co-ordinates to UTM using the WGS84 ellipsoid.
        ### Params
         * **ll**: `LatLon` struct with lat and lon properties representing the WGS84 coordinate to be converted.
        ### Return
         * `Utm` struct containing the UTM value.
        */

        let lat = ll.lat;
        let lon = ll.lon;
        let a = 6378137.0; //ellip.radius;
        let ecc_sq = 0.00669438; //ellip.eccsq;
        let k0 = 0.9996;
        let lon_origin;
        let ecc_prm_sq;
        let lat_rad = lat.to_radians();
        let lon_rad = lon.to_radians();
        let lon_origin_rad;
        let mut zone_num = f64::floor((lon + 180.0) / 6.0) + 1.0;

        //Make sure the lonitude 180.00 is in Zone 60
        if lon == 180.0 {
            zone_num = 60.0;
        }

        // Special zone for Norway
        if lat >= 56.0 && lat < 64.0 && lon >= 3.0 && lon < 12.0 {
            zone_num = 32.0;
        }

        // Special zones for Svalbard
        if lat >= 72.0 && lat < 84.0 {
            if lon >= 0.0 && lon < 9.0 {
                zone_num = 31.0;
            } else if lon >= 9.0 && lon < 21.0 {
                zone_num = 33.0;
            } else if lon >= 21.0 && lon < 33.0 {
                zone_num = 35.0;
            } else if lon >= 33.0 && lon < 42.0 {
                zone_num = 37.0;
            }
        }

        lon_origin = (zone_num - 1.0) * 6.0 - 180.0 + 3.0; //+3 puts origin in middle of zone
        lon_origin_rad = lon_origin.to_radians();

        ecc_prm_sq = (ecc_sq) / (1.0 - ecc_sq);

        let n = a / f64::sqrt(1.0 - ecc_sq * lat_rad.sin() * lat_rad.sin());
        let t = lat_rad.tan() * lat_rad.tan();
        let c = ecc_prm_sq * lat_rad.cos() * lat_rad.cos();
        let a = lat_rad.cos() * (lon_rad - lon_origin_rad);

        let m = a *
            (1.0 - ecc_sq / 4.0 - 3.0 * ecc_sq * ecc_sq / 64.0 - 5.0 * ecc_sq * ecc_sq * ecc_sq / 256.0) *
            lat_rad -
            (3.0 * ecc_sq / 8.0 + 3.0 * ecc_sq * ecc_sq / 32.0 + 45.0 * ecc_sq * ecc_sq * ecc_sq / 1024.0) *
            f64::sin(2.0 * lat_rad) +
            (15.0 * ecc_sq * ecc_sq / 256.0 + 45.0 * ecc_sq * ecc_sq * ecc_sq / 1024.0) *
            f64::sin(4.0 * lat_rad) -
            (35.0 * ecc_sq * ecc_sq * ecc_sq / 3072.0) *
            f64::sin(6.0 * lat_rad);

        let utm_e = k0 *
            n *
            (a +
                (1.0 - t + c) * a * a * a / 6.0 +
                (5.0 - 18.0 * t + t * t + 72.0 * c - 58.0 * ecc_prm_sq) *
            a * a * a * a * a / 120.0) +
            500000.0;

        let mut utm_n = k0 *
            (m + n * lat_rad.tan() *
                (a * a / 2.0 +
                    (5.0 - t + 9.0 * c + 4.0 * c * c) * a * a * a * a / 24.0 +
                    (61.0 - 58.0 * t + t * t + 600.0 * c - 330.0 * ecc_prm_sq) *
                    a * a * a * a * a * a / 720.0));
        if lat < 0.0 {
            utm_n += 10000000.0; //10000000 meter offset for
            // southern hemisphere
        }

        Utm {
            n: utm_n.round(),
            e: utm_e.round(),
            gzd: Gzd { num: zone_num as u8, letter: LatBand::from(lat) }
        }
    }

    pub fn to_mgrs(self, accuracy: Option<Accuracy>) -> Mgrs {
        /*!
        Converts UTM coordinate to MGRS reference.

         @returns {Mgrs}
         @throws  {Error} Invalid coordinate

         @example
           var utmCoord = new Utm(31, 'N', 448251, 5411932);
           var mgrsRef = utmCoord.toMgrs(); // mgrsRef.toString() = '31U DQ 48251 11932'
        */

        let e100k = ColLetter::from_zone_and_easting(self.zone, self.easting);

        let n100k = RowLetter::from_zone_and_northing(self.zone, self.northing);

        // truncate easting/northing to within 100km grid square and round to nm precision
        // round to reasonable precision
        let to_precision = |x: i32, y: u32| -> usize {
            let p = i32::pow(10, y);
            (((x % p) * p) / p) as usize
        };

        Mgrs {
            gzd: Gzd { zone: self.zone, band: LatBand::from_lat(self.as_ll_e().lat) },
            gsid_100k: GridSquareId100k { col: e100k, row: n100k },
            easting: to_precision(self.easting, 6),
            northing: to_precision(self.northing, 6),
        }
    }

    pub fn get_100k_id(&self) -> GridSquareId100k {
        /*!
        Get the two letter 100k designator for a given UTM easting,
        northing and zone number value.

        ### Params
         * **easting**
         * **northing
         * **zone_num
        ### Return
         * the two letter 100k designator for the given UTM location.
        */
        use std::f64;
        let set_parm = get_100k_set_for_zone(self.gzd.num as usize);
        let set_column = f64::floor(self.easting / 100000.0) as u32;
        let set_row = (f64::floor(self.northing / 100000.0) % 20.0) as u32;
        GridSquareId100k::new(set_column, set_row, set_parm)
    }

    fn as_string(&self, digits: usize) -> String {
        /*!
        Returns a string representation of a UTM coordinate.

        To distinguish from MGRS grid zone designators, a space is left between the zone and the
        hemisphere.

        ### Params
         * **digits** Determines the number of digits to return after the decimal

        ### Returns
         * A string representation of the coordinate to the specified precision of `digits`.
        */

        format!("{} {} {2:.4$} {3:.4$}", self.zone, self.hemisphere, self.easting, self.northing, digits)
    }
}

impl From<LatLon> for Utm {
    fn from(ll: LatLon) -> Self {
        Utm::from_ll(&ll)
    }
}

impl fmt::Display for Utm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.as_string(5))
    }
}
