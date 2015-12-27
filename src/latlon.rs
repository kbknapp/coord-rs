use Lat;
use Lon;
use Utm;
use Mgrs;
use Accuracy;
use Gzd;
use band::LatBand;
use errors::Errors;
use hemisphere::Hemisphere;
use datum::Datum;

#[derive(Copy, Clone, Debug, Default)]
pub struct LatLon {
    pub lat: f64,
    pub lon: f64,
}

impl LatLon {
    pub fn new(lat: f64, lon: f64) -> Result<Self, Errors> {
        if (!(-80.0 <= lat && lat <= 84.0)) {
            return Err(Errors::InvalidLatitude(lat));
        }
        Ok(LatLon {
            lat: lat,
            lon: lon
        })
    }

    pub fn to_utm(self) -> Utm {
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

        let mut zone = (f64::floor((self.lon + 180.0) / 6.0) + 1.0) as u8; // longitudinal zone
        let mut lamda0 = f64::to_radians(((zone - 1) * 6 - 180 + 3) as f64); // longitude of central meridian

        // ---- handle Norway/Svalbard exceptions
        // grid zones are 8° tall; 0°N is offset 10 into latitude bands array
        let mgrs_lat_bands = b"CDEFGHJKLMNPQRSTUVWXX"; // X is repeated for 80-84°N
        let lat_band = mgrs_lat_bands[f64::floor(self.lat / 8.0 + 10.0) as usize];

        // adjust zone & central meridian for Norway
        if zone == 31 && lat_band == b'V' && self.lon >= 3.0 { zone += 1; lamda0 += f64::to_radians(6.0); }
        // adjust zone & central meridian for Svalbard
        if (zone == 32 && lat_band == b'X' && self.lon <  9.0)  { zone -= 1; lamda0 -= f64::to_radians(6.0); }
        if (zone == 32 && lat_band == b'X' && self.lon >= 9.0)  { zone += 1; lamda0 += f64::to_radians(6.0); }
        if (zone == 34 && lat_band == b'X' && self.lon <  21.0) { zone -= 1; lamda0 -= f64::to_radians(6.0); }
        if (zone == 34 && lat_band == b'X' && self.lon >= 21.0) { zone += 1; lamda0 += f64::to_radians(6.0); }
        if (zone == 36 && lat_band == b'X' && self.lon <  33.0) { zone -= 1; lamda0 -= f64::to_radians(6.0); }
        if (zone == 36 && lat_band == b'X' && self.lon >= 33.0) { zone += 1; lamda0 += f64::to_radians(6.0); }

        let phi = f64::to_radians(self.lat);      // latitude ± from equator
        let lamda = f64::to_radians(self.lon) - lamda0; // longitude ± from central meridian

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
            (f64::round((x * p) as f64) / p as f64) as i32
        };

        Utm {
            zone: zone,
            hemisphere: Hemisphere::from(self.lat),
            easting: to_precision(x % 100000, 6), // nm precision,
            northing: to_precision(y % 100000, 6),
            datum: self.datum,
            convergence: to_precision(gamma.to_degrees(), 9),
            scale: to_precision(k, 12),
        }
    }

    pub fn from_utm(utm: &Utm) -> Option<Self> {
        /*!
        Converts UTM coords to lat/lon, using the WGS84 ellipsoid. This is a convenience
        class where the Zone can be specified as a single string eg."60N" which
        is then broken down into the zone_num and ZoneLetter.

        ### Params
         * **utm**: `Utm` struct If an optional accuracy property is
            provided (in meters), a bounding box will be returned instead of
            latitude and lonitude.
        ### Return
         * `LatLon` containing either lat and lon values
            (if no accuracy was provided), or top, right, bottom and left values
            for the bounding box calculated according to the provided accuracy. Returns `None` if
            if the conversion fails.
        */


        let utm_n= utm.n;
        let utm_e= utm.e;
        let zone_letter = utm.gzd.letter;
        let zone_num = utm.gzd.num;

        let k0 = 0.9996;
        let a = 6378137.0; //ellip.radius;
        let ecc_sq = 0.00669438; //ellip.eccsq;
        let e1 = (1.0 - f64::sqrt(1.0 - ecc_sq)) / (1.0 + f64::sqrt(1.0 - ecc_sq));

        // remove 500,000 meter offset for lonitude
        let x = utm_e - 500000.0;
        let mut y = utm_n;

        // We must know somehow if we are in the Northern or Southern hemisphere, this is the only
        // time we use the letter So even if the Zone letter isn't exactly correct it should indicate
        // the hemisphere correctly
        if zone_letter < LatBand::from('N') {
            // remove 10,000,000 meter offset used for southern hemisphere
            y -= 10000000.0;
        }

      // There are 60 zones with zone 1 being at West -180 to -174
      let lon_origin = ((zone_num - 1) * 6 - 180 + 3) as f64; // +3 puts origin in middle of zone

      let ecc_prm_sq = (ecc_sq) / (1.0 - ecc_sq);

      let m = y / k0;
      let mu = m /
        (a *
            (1.0 - ecc_sq / 4.0 - 3.0 * ecc_sq * ecc_sq / 64.0 - 5.0 * ecc_sq * ecc_sq * ecc_sq / 256.0));

      let phi1_rad = mu +
        (3.0 * e1 / 2.0 - 27.0 * e1 * e1 * e1 / 32.0) *
        f64::sin(2.0 * mu) +
        (21.0 * e1 * e1 / 16.0 - 55.0 * e1 * e1 * e1 * e1 / 32.0) *
        f64::sin(4.0 * mu) +
        (151.0 * e1 * e1 * e1 / 96.0) *
        f64::sin(6.0 * mu);
      // double phi1 = ProjradToDeg(phi1_rad);

      let n1 = a / f64::sqrt(1.0 - ecc_sq * phi1_rad.sin() * phi1_rad.sin());
      let t1 = phi1_rad.tan() * phi1_rad.tan();
      let c1 = ecc_prm_sq * phi1_rad.cos() * phi1_rad.cos();
      let r1 = a * (1.0 - ecc_sq) / f64::powf(1.0 - ecc_sq * phi1_rad.sin() * phi1_rad.sin(), 1.5);
      let d = x / (n1 * k0);

      let mut lat = phi1_rad -
        (n1 * phi1_rad.tan() / r1) *
        (d * d / 2.0 -
            (5.0 + 3.0 * t1 + 10.0 * c1 - 4.0 * c1 * c1 - 9.0 * ecc_prm_sq) *
            d * d * d * d / 24.0 +
            (61.0 + 90.0 * t1 + 298.0 * c1 + 45.0 * t1 * t1 - 252.0 * ecc_prm_sq - 3.0 * c1 * c1) *
            d * d * d * d * d * d / 720.0);
      lat = lat.to_degrees();

      let mut lon = (d -
          (1.0 + 2.0 * t1 + c1) *
          d * d * d / 6.0 +
          (5.0 - 2.0 * c1 + 28.0 * t1 - 3.0 * c1 * c1 + 8.0 * ecc_prm_sq + 24.0 * t1 * t1) *
          d * d * d * d * d / 120.0) /
          phi1_rad.cos();

      lon = lon_origin + lon.to_degrees();

        Some(LatLon {
            lat: lat,
            lon: lon
        })
    }

    pub fn from_mgrs<M: Into<Mgrs>>(mgrs: M) -> LatLon {
        let m = mgrs.into();
        m.to_ll()
    }

    pub fn rect_from_mgrs<M: Into<Mgrs>>(m: M) -> Option<[LatLon; 2]> {
        let mgrs = m.into();
        let bl = LatLon::from_utm(&mgrs.utm).expect("failed to convert MGRS to Lat/Lon");

        let tr = LatLon::from(
            Utm {
                n: mgrs.utm.n + mgrs.accuracy.as_numeric() as f64,
                e: mgrs.utm.e + mgrs.accuracy.as_numeric() as f64,
                gzd: Gzd { num: mgrs.utm.gzd.num, letter: mgrs.utm.gzd.letter }
            }
        );
        Some([tr, bl])
    }

    pub fn to_mgrs(self, acc: Option<Accuracy>) -> Mgrs {
        /*!
        Conversion of lat/lon to MGRS.

        ### Params
         * **ll**: `latLon` struct with lat and lon properties on a WGS84 ellipsoid.
         * **accuracy**: an optional `Accuracy`, default is `Accuracy::m1` (1 meter).
        ### Return
         * the `Mgrs` struct for the given location and accuracy.
        */
        Utm::from(self).to_mgrs(acc)
    }

    pub fn as_mgrs(&self, acc: Option<Accuracy>) -> Mgrs {
        /*!
        Conversion of lat/lon to MGRS.

        ### Params
         * **ll**: `latLon` struct with lat and lon properties on a WGS84 ellipsoid.
         * **accuracy**: an optional `Accuracy`, default is `Accuracy::m1` (1 meter).
        ### Return
         * the `Mgrs` struct for the given location and accuracy.
        */
        Utm::from_ll(self).to_mgrs(acc)
    }
}

impl From<Utm> for LatLon {
    fn from(u: Utm) -> Self {
        match LatLon::from_utm(&u) {
            Some(l) => l,
            None => panic!("Couldn't convert UTM to Lat/Lon"),
        }
    }
}
