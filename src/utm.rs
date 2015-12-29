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
        Converts latitude/longitude to UTM coordinate.

        Implements Karney’s method, using Krüger series to order n^6, giving results accurate to 5nm for
        distances up to 3900km from the central meridian.

        @returns {Utm}   UTM coordinate.
        @throws  {Error} If point not valid, if point outside latitude range.

        @example
          var latlong = new LatLon(48.8582, 2.2945, LatLon.datum.WGS84);
          var utmCoord = latlong.toUtm(); // utmCoord.toString(): '31 N 448252 5411933'
        */

        let false_easting = 500e3;
        let false_northing = 10000e3;

        let mut zone = (f64::floor((ll.lon + 180.0) / 6.0) + 1.0) as u8; // longitudinal zone
        let mut lamda0 = f64::to_radians(((zone - 1) * 6 - 180 + 3) as f64); // longitude of central meridian

        // ---- handle Norway/Svalbard exceptions
        // grid zones are 8° tall; 0°N is offset 10 into latitude bands array
        let mgrs_lat_bands = b"CDEFGHJKLMNPQRSTUVWXX"; // X is repeated for 80-84°N
        let lat_band = mgrs_lat_bands[f64::floor(ll.lat / 8.0 + 10.0) as usize];

        // adjust zone & central meridian for Norway
        if zone == 31 && lat_band == b'V' && ll.lon >= 3.0 { zone += 1; lamda0 += f64::to_radians(6.0); }
        // adjust zone & central meridian for Svalbard
        if (zone == 32 && lat_band == b'X' && ll.lon <  9.0)  { zone -= 1; lamda0 -= f64::to_radians(6.0); }
        if (zone == 32 && lat_band == b'X' && ll.lon >= 9.0)  { zone += 1; lamda0 += f64::to_radians(6.0); }
        if (zone == 34 && lat_band == b'X' && ll.lon <  21.0) { zone -= 1; lamda0 -= f64::to_radians(6.0); }
        if (zone == 34 && lat_band == b'X' && ll.lon >= 21.0) { zone += 1; lamda0 += f64::to_radians(6.0); }
        if (zone == 36 && lat_band == b'X' && ll.lon <  33.0) { zone -= 1; lamda0 -= f64::to_radians(6.0); }
        if (zone == 36 && lat_band == b'X' && ll.lon >= 33.0) { zone += 1; lamda0 += f64::to_radians(6.0); }

        let phi = f64::to_radians(ll.lat);      // latitude ± from equator
        let lamda = f64::to_radians(ll.lon) - lamda0; // longitude ± from central meridian

        // WGS 84: a = 6378137, b = 6356752.314245, f = 1/298.257223563;
        let a = Datum::Wgs84.a();
        let f = Datum::Wgs84.f();

        let k0 = 0.9996; // UTM scale on the central meridian

        // ---- easting, northing: Karney 2011 Eq 7-14, 29, 35:

        let e = f64::sqrt(f * (2.0 - f)); // eccentricity
        let n = f / (2.0 - f);        // 3rd flattening
        let n2 = n * n;
        let n3 = n * n2;
        let n4 = n * n3;
        let n5 = n * n4;
        let n6 = n * n5; // TODO: compare Horner-form accuracy?

        let coslamda = lamda.cos();
        let sinlamda = lamda.sin();
        let tanlamda = lamda.tan();

        // tau ≡ tanphi, tau2 ≡ tanphi2; prime (2) indicates angles on the conformal sphere
        let tau = phi.tan();
        let delta = f64::sinh(e * f64::atanh(e * tau / f64::sqrt( 1.0 + tau * tau)));

        let tau2 = tau * f64::sqrt( 1.0 + delta * delta) - delta * f64::sqrt( 1.0 + tau * tau);

        let xi = f64::atan2(tau2, coslamda);
        let eta = f64::asinh(sinlamda / f64::sqrt(tau2 * tau2 + coslamda * coslamda));

        // 2πA is the circumference of a meridian
        let A = a / (1.0 + n) * (1.0 + 1.0 / 4.0 * n2 + 1.0 / 64.0 * n4 + 1.0 / 256.0 * n6);

        // note alpha is one-based array (6th order Krüger expressions)
        let alpha = [ 0.0,
            1.0 / 2.0 *n - 2.0 / 3.0 * n2 + 5.0 / 16.0 * n3 + 41.0 / 180.0 * n4 - 127.0 / 288.0 * n5 + 7891.0 / 37800.0 * n6,
            13.0 / 48.0 * n2 - 3.0 / 5.0 * n3 + 557.0 / 1440.0 * n4 + 281.0 / 630.0 * n5 - 1983433.0 / 1935360.0 * n6,
            61.0 / 240.0 * n3 - 103.0 / 140.0 * n4 + 15061.0 / 26880.0 * n5 + 167603.0 / 181440.0 * n6,
            49561.0 / 161280.0 * n4 - 179.0 / 168.0 * n5 + 6601661.0 / 7257600.0 * n6,
            34729.0 / 80640.0 * n5 - 3418889.0 / 1995840.0 * n6,
            212378941.0 / 319334400.0 * n6 ];

        let xi2 = xi;
        for j in 1..6 { xi += alpha[j] * f64::sin(2.0 * j as f64 * xi2) * f64::cosh(2.0 * j as f64 * eta); }

        let eta2 = eta;
        for j in 1..6 { eta += alpha[j] * f64::cos(2.0 * j as f64 * xi2) * f64::sinh(2.0 * j as f64 *eta2); }

        let x = k0 * A * eta;
        let y = k0 * A * xi;

        // ---- convergence: Karney 2011 Eq 23, 24

        let p2 = 1.0;
        for j in 1..6 { p2 += 2.0 * j as f64 * alpha[j] * f64::cos( 2.0 * j as f64 * xi2) * f64::cosh(2.0 * j as f64 * eta2); }
        let q2 = 0.0;
        for j in 1..6 { q2 += 2.0 * j as f64 * alpha[j] * f64::sin(2.0 * j as f64 * xi2) * f64::sinh(2.0 * j as f64 * eta2); }

        let gamma2 = f64::atan(tau2 / f64::sqrt(1.0 + tau2 * tau2) * tanlamda);
        let gamma3 = q2.atan2(p2);

        let gamma = gamma2 + gamma3;

        // ---- scale: Karney 2011 Eq 25

        let sinphi = phi.sin();
        let k2 = f64::sqrt(1.0 - e * e * sinphi * sinphi) * f64::sqrt(1.0 + tau * tau) / f64::sqrt(tau2 * tau2 + coslamda * coslamda);
        let k3 = A / a * f64::sqrt(p2 * p2 + q2 * q2);

        let k = k0 * k2 * k3;

        // ------------

        // shift x/y to false origins
        x = x + false_easting;             // make x relative to false easting
        if y < 0.0 { y = y + false_northing; } // make y in southern hemisphere relative to false northing

        // round to reasonable precision
        let to_precision = |x: i32, y: u32| -> i32 {
            let p = i32::pow(10, y);
            (x * p) as i32 / p as i32
        };
        let to_precisionf = |x: f64, y: f64| -> f64 {
            let p = f64::powf(10.0, y);
            f64::round(x * p) / p
        };

        Utm {
            zone: zone,
            hemisphere: Hemisphere::from(ll.lat),
            easting: to_precision(x as i32 % 100000, 6), // nm precision,
            northing: to_precision(y as i32 % 100000, 6),
            datum: ll.datum,
            convergence: Some(to_precisionf(gamma.to_degrees(), 9.0)),
            scale: Some(to_precisionf(k, 12.0)),
        }
    }

    fn from_mgrs(mgrs: Mgrs) -> Self {
        /*!
        Converts MGRS grid reference to UTM coordinate.

        ### Returns
         * A `Utm` struct

        # Examples

        ```
        let mgrs = Mgrs::from("31U DQ 448251 11932");
        let utm = mgrs.as_utm();
        assert_eq!(&*utm.as_string(6), "31 N 448251 541193");
        ```
        */

        // get easting specified by e100k
        let e100k_num = mgrs.gsid_100k.col.as_meters_from_zone(mgrs.gzd.zone);

        // get northing specified by n100k
        let n100k_num = mgrs.gsid_100k.row.as_meters_from_zone(mgrs.gzd.zone);

        // get latitude of (bottom of) band
        let lat_band: f64 = mgrs.gzd.band.into();

        // 100km grid square row letters repeat every 2,000km north; add enough 2,000km blocks to get
        // into required band
        let utm: Utm = LatLon::new(lat_band, 0.0).unwrap().into();
        let n_band = utm.northing; // northing of bottom of band
        let mut n2m = 0; // northing of 2,000km block
        while (n2m + n100k_num + mgrs.northing) < n_band { n2m += 2000000; }

        Utm::new(mgrs.gzd.zone, mgrs.gzd.band, e100k_num + mgrs.easting, n2m + n100k_num + mgrs.northing)
    }

    // pub fn from_ll(ll: &LatLon) -> Self {
    //     /*!
    //     Converts a set of lonitude and latitude co-ordinates to UTM using the WGS84 ellipsoid.
    //     ### Params
    //      * **ll**: `LatLon` struct with lat and lon properties representing the WGS84 coordinate to be converted.
    //     ### Return
    //      * `Utm` struct containing the UTM value.
    //     */
    //
    //     let lat = ll.lat;
    //     let lon = ll.lon;
    //     let a = 6378137.0; //ellip.radius;
    //     let ecc_sq = 0.00669438; //ellip.eccsq;
    //     let k0 = 0.9996;
    //     let lon_origin;
    //     let ecc_prm_sq;
    //     let lat_rad = lat.to_radians();
    //     let lon_rad = lon.to_radians();
    //     let lon_origin_rad;
    //     let mut zone_num = f64::floor((lon + 180.0) / 6.0) + 1.0;
    //
    //     //Make sure the lonitude 180.00 is in Zone 60
    //     if lon == 180.0 {
    //         zone_num = 60.0;
    //     }
    //
    //     // Special zone for Norway
    //     if lat >= 56.0 && lat < 64.0 && lon >= 3.0 && lon < 12.0 {
    //         zone_num = 32.0;
    //     }
    //
    //     // Special zones for Svalbard
    //     if lat >= 72.0 && lat < 84.0 {
    //         if lon >= 0.0 && lon < 9.0 {
    //             zone_num = 31.0;
    //         } else if lon >= 9.0 && lon < 21.0 {
    //             zone_num = 33.0;
    //         } else if lon >= 21.0 && lon < 33.0 {
    //             zone_num = 35.0;
    //         } else if lon >= 33.0 && lon < 42.0 {
    //             zone_num = 37.0;
    //         }
    //     }
    //
    //     lon_origin = (zone_num - 1.0) * 6.0 - 180.0 + 3.0; //+3 puts origin in middle of zone
    //     lon_origin_rad = lon_origin.to_radians();
    //
    //     ecc_prm_sq = (ecc_sq) / (1.0 - ecc_sq);
    //
    //     let n = a / f64::sqrt(1.0 - ecc_sq * lat_rad.sin() * lat_rad.sin());
    //     let t = lat_rad.tan() * lat_rad.tan();
    //     let c = ecc_prm_sq * lat_rad.cos() * lat_rad.cos();
    //     let a = lat_rad.cos() * (lon_rad - lon_origin_rad);
    //
    //     let m = a *
    //         (1.0 - ecc_sq / 4.0 - 3.0 * ecc_sq * ecc_sq / 64.0 - 5.0 * ecc_sq * ecc_sq * ecc_sq / 256.0) *
    //         lat_rad -
    //         (3.0 * ecc_sq / 8.0 + 3.0 * ecc_sq * ecc_sq / 32.0 + 45.0 * ecc_sq * ecc_sq * ecc_sq / 1024.0) *
    //         f64::sin(2.0 * lat_rad) +
    //         (15.0 * ecc_sq * ecc_sq / 256.0 + 45.0 * ecc_sq * ecc_sq * ecc_sq / 1024.0) *
    //         f64::sin(4.0 * lat_rad) -
    //         (35.0 * ecc_sq * ecc_sq * ecc_sq / 3072.0) *
    //         f64::sin(6.0 * lat_rad);
    //
    //     let utm_e = k0 *
    //         n *
    //         (a +
    //             (1.0 - t + c) * a * a * a / 6.0 +
    //             (5.0 - 18.0 * t + t * t + 72.0 * c - 58.0 * ecc_prm_sq) *
    //         a * a * a * a * a / 120.0) +
    //         500000.0;
    //
    //     let mut utm_n = k0 *
    //         (m + n * lat_rad.tan() *
    //             (a * a / 2.0 +
    //                 (5.0 - t + 9.0 * c + 4.0 * c * c) * a * a * a * a / 24.0 +
    //                 (61.0 - 58.0 * t + t * t + 600.0 * c - 330.0 * ecc_prm_sq) *
    //                 a * a * a * a * a * a / 720.0));
    //     if lat < 0.0 {
    //         utm_n += 10000000.0; //10000000 meter offset for
    //         // southern hemisphere
    //     }
    //
    //     Utm {
    //         n: utm_n.round(),
    //         e: utm_e.round(),
    //         gzd: Gzd { num: zone_num as u8, letter: LatBand::from(lat) }
    //     }
    // }

    // pub fn to_mgrs(self, accuracy: Option<Accuracy>) -> Mgrs {
    //     /*!
    //     Converts UTM coordinate to MGRS reference.
    //
    //      @returns {Mgrs}
    //      @throws  {Error} Invalid coordinate
    //
    //      @example
    //        var utmCoord = new Utm(31, 'N', 448251, 5411932);
    //        var mgrsRef = utmCoord.toMgrs(); // mgrsRef.toString() = '31U DQ 48251 11932'
    //     */
    //
    //     let e100k = ColLetter::from_zone_and_easting(self.zone, self.easting);
    //
    //     let n100k = RowLetter::from_zone_and_northing(self.zone, self.northing);
    //
    //     // truncate easting/northing to within 100km grid square and round to nm precision
    //     // round to reasonable precision
    //     let to_precision = |x: i32, y: u32| -> usize {
    //         let p = i32::pow(10, y);
    //         (((x % p) * p) / p) as usize
    //     };
    //
    //     Mgrs {
    //         gzd: Gzd { zone: self.zone, band: LatBand::from(self.as_ll_e().lat) },
    //         gsid_100k: GridSquareId100k { col: e100k, row: n100k },
    //         easting: to_precision(self.easting, 6),
    //         northing: to_precision(self.northing, 6),
    //     }
    // }

    // pub fn get_100k_id(&self) -> GridSquareId100k {
    //     /*!
    //     Get the two letter 100k designator for a given UTM easting,
    //     northing and zone number value.
    //
    //     ### Params
    //      * **easting**
    //      * **northing
    //      * **zone_num
    //     ### Return
    //      * the two letter 100k designator for the given UTM location.
    //     */
    //     use std::f64;
    //     let set_parm = get_100k_set_for_zone(self.num as usize);
    //     let set_column = f64::floor(self.easting / 100000) as u32;
    //     let set_row = (f64::floor(self.northing / 100000) % 20.0) as u32;
    //     GridSquareId100k::new(set_column, set_row, set_parm)
    // }

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
