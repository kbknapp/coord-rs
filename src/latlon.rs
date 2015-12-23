use Lat;
use Lon;
use Utm;
use Mgrs;
use Accuracy;
use Gzd;
use band::LatBand;

#[derive(Copy, Clone, Debug, Default)]
pub struct LatLon {
    pub lat: Lat,
    pub lon: Lon,
}

impl LatLon {
    pub fn new(lat: Lat, lon: Lon) -> Self {
        LatLon {
            lat: lat,
            lon: lon
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
