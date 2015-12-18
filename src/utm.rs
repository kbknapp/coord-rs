use gzd::{Gzd, GridSquareId100k};
use get_100k_set_for_zone;
use latlon::LatLon;
use Mgrs;
use Accuracy;
use ZoneLetter;

#[derive(Default, Copy, Clone, Debug)]
pub struct Utm {
    /// Grid Zone Designator such as 26F
    pub gzd: Gzd,
    /// easting
    pub e: f64,
    /// northing
    pub n: f64
}

impl Utm {
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
            gzd: Gzd { num: zone_num as u8, letter: ZoneLetter::from(lat) }
        }
    }

    pub fn to_mgrs(self, accuracy: Option<Accuracy>) -> Mgrs {
        /*!
        Converts a UTM location to an MGRS struct and consumes itself.

        ### Params
         * **accuracy**: `Accuracy` enum.
        ### Return
         * `Mgrs` struct for the given UTM location.
        */
        Mgrs {
            gsid_100k: self.get_100k_id(),
            utm: self,
            accuracy: accuracy.unwrap_or(Accuracy::One)
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
        let set_column = f64::floor(self.e / 100000.0) as u32;
        let set_row = (f64::floor(self.n / 100000.0) % 20.0) as u32;
        GridSquareId100k::new(set_column, set_row, set_parm)
    }
}

impl From<LatLon> for Utm {
    fn from(ll: LatLon) -> Self {
        Utm::from_ll(&ll)
    }
}
