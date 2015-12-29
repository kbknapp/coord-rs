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
    /// Datum UTM coordinate is based on.
    pub datum: Datum,
    /// Meridian convergence (bearing of grid north clockwise from true north), in degrees
    pub convergence: Option<f64>,
    /// Grid scale factor
    pub scale: Option<f64>,
}

impl LatLon {
    pub fn new(lat: f64, lon: f64) -> Result<Self, Errors> {
        if (!(-80.0 <= lat && lat <= 84.0)) {
            return Err(Errors::InvalidLatitude(lat));
        }
        Ok(LatLon {
            lat: lat,
            lon: lon,
            datum: Datum::Wgs84,
            scale: None,
            convergence: None,
        })
    }

    // pub fn from_utm(utm: &Utm) -> Option<Self> {
    //     /*!
    //     Converts UTM coords to lat/lon, using the WGS84 ellipsoid. This is a convenience
    //     class where the Zone can be specified as a single string eg."60N" which
    //     is then broken down into the zone_num and ZoneLetter.
    //
    //     ### Params
    //      * **utm**: `Utm` struct If an optional accuracy property is
    //         provided (in meters), a bounding box will be returned instead of
    //         latitude and lonitude.
    //     ### Return
    //      * `LatLon` containing either lat and lon values
    //         (if no accuracy was provided), or top, right, bottom and left values
    //         for the bounding box calculated according to the provided accuracy. Returns `None` if
    //         if the conversion fails.
    //     */
    //
    //
    //     let utm_n= utm.n;
    //     let utm_e= utm.e;
    //     let zone_letter = utm.gzd.letter;
    //     let zone_num = utm.gzd.num;
    //
    //     let k0 = 0.9996;
    //     let a = 6378137.0; //ellip.radius;
    //     let ecc_sq = 0.00669438; //ellip.eccsq;
    //     let e1 = (1.0 - f64::sqrt(1.0 - ecc_sq)) / (1.0 + f64::sqrt(1.0 - ecc_sq));
    //
    //     // remove 500,000 meter offset for lonitude
    //     let x = utm_e - 500000.0;
    //     let mut y = utm_n;
    //
    //     // We must know somehow if we are in the Northern or Southern hemisphere, this is the only
    //     // time we use the letter So even if the Zone letter isn't exactly correct it should indicate
    //     // the hemisphere correctly
    //     if zone_letter < LatBand::from('N') {
    //         // remove 10,000,000 meter offset used for southern hemisphere
    //         y -= 10000000.0;
    //     }
    //
    //   // There are 60 zones with zone 1 being at West -180 to -174
    //   let lon_origin = ((zone_num - 1) * 6 - 180 + 3) as f64; // +3 puts origin in middle of zone
    //
    //   let ecc_prm_sq = (ecc_sq) / (1.0 - ecc_sq);
    //
    //   let m = y / k0;
    //   let mu = m /
    //     (a *
    //         (1.0 - ecc_sq / 4.0 - 3.0 * ecc_sq * ecc_sq / 64.0 - 5.0 * ecc_sq * ecc_sq * ecc_sq / 256.0));
    //
    //   let phi1_rad = mu +
    //     (3.0 * e1 / 2.0 - 27.0 * e1 * e1 * e1 / 32.0) *
    //     f64::sin(2.0 * mu) +
    //     (21.0 * e1 * e1 / 16.0 - 55.0 * e1 * e1 * e1 * e1 / 32.0) *
    //     f64::sin(4.0 * mu) +
    //     (151.0 * e1 * e1 * e1 / 96.0) *
    //     f64::sin(6.0 * mu);
    //   // double phi1 = ProjradToDeg(phi1_rad);
    //
    //   let n1 = a / f64::sqrt(1.0 - ecc_sq * phi1_rad.sin() * phi1_rad.sin());
    //   let t1 = phi1_rad.tan() * phi1_rad.tan();
    //   let c1 = ecc_prm_sq * phi1_rad.cos() * phi1_rad.cos();
    //   let r1 = a * (1.0 - ecc_sq) / f64::powf(1.0 - ecc_sq * phi1_rad.sin() * phi1_rad.sin(), 1.5);
    //   let d = x / (n1 * k0);
    //
    //   let mut lat = phi1_rad -
    //     (n1 * phi1_rad.tan() / r1) *
    //     (d * d / 2.0 -
    //         (5.0 + 3.0 * t1 + 10.0 * c1 - 4.0 * c1 * c1 - 9.0 * ecc_prm_sq) *
    //         d * d * d * d / 24.0 +
    //         (61.0 + 90.0 * t1 + 298.0 * c1 + 45.0 * t1 * t1 - 252.0 * ecc_prm_sq - 3.0 * c1 * c1) *
    //         d * d * d * d * d * d / 720.0);
    //   lat = lat.to_degrees();
    //
    //   let mut lon = (d -
    //       (1.0 + 2.0 * t1 + c1) *
    //       d * d * d / 6.0 +
    //       (5.0 - 2.0 * c1 + 28.0 * t1 - 3.0 * c1 * c1 + 8.0 * ecc_prm_sq + 24.0 * t1 * t1) *
    //       d * d * d * d * d / 120.0) /
    //       phi1_rad.cos();
    //
    //   lon = lon_origin + lon.to_degrees();
    //
    //     Some(LatLon {
    //         lat: lat,
    //         lon: lon
    //     })
    // }

    pub fn from_mgrs<M: Into<Mgrs>>(mgrs: M) -> Self {
        let m = mgrs.into();
        m.to_ll()
    }

    // pub fn rect_from_mgrs<M: Into<Mgrs>>(m: M) -> Option<[LatLon; 2]> {
    //     let mgrs = m.into();
    //     let bl = LatLon::from_utm(&mgrs.utm).expect("failed to convert MGRS to Lat/Lon");
    //
    //     let tr = LatLon::from(
    //         Utm {
    //             n: mgrs.utm.n + mgrs.accuracy.as_numeric() as f64,
    //             e: mgrs.utm.e + mgrs.accuracy.as_numeric() as f64,
    //             gzd: Gzd { zone: mgrs.gzd.zone, band: mgrs.utm.gzd.letter }
    //         }
    //     );
    //     Some([tr, bl])
    // }

    // pub fn to_mgrs(self, acc: Option<Accuracy>) -> Mgrs {
    //     /*!
    //     Conversion of lat/lon to MGRS.
    //
    //     ### Params
    //      * **ll**: `latLon` struct with lat and lon properties on a WGS84 ellipsoid.
    //      * **accuracy**: an optional `Accuracy`, default is `Accuracy::m1` (1 meter).
    //     ### Return
    //      * the `Mgrs` struct for the given location and accuracy.
    //     */
    //     Utm::from(self).to_mgrs(acc)
    // }
    //
    // pub fn as_mgrs(&self, acc: Option<Accuracy>) -> Mgrs {
    //     /*!
    //     Conversion of lat/lon to MGRS.
    //
    //     ### Params
    //      * **ll**: `latLon` struct with lat and lon properties on a WGS84 ellipsoid.
    //      * **accuracy**: an optional `Accuracy`, default is `Accuracy::m1` (1 meter).
    //     ### Return
    //      * the `Mgrs` struct for the given location and accuracy.
    //     */
    //     Utm::from_ll(self).to_mgrs(acc)
    // }
}

impl From<Utm> for LatLon {
    fn from(utm: Utm) -> Self {
        /*!
        Converts UTM zone/easting/northing coordinate to latitude/longitude

        @param   {Utm}     utmCoord - UTM coordinate to be converted to latitude/longitude.
        @returns {LatLon} Latitude/longitude of supplied grid reference.

        @example
          let grid = new Utm(31, 'N', 448251.795, 5411932.678);
          let latlong = grid.toLatLonE(); // latlong.toString(): 48°51′29.52″N, 002°17′40.20″E
        */
        let z = utm.zone;
        let h = utm.hemisphere;
        let x = utm.easting;
        let y = utm.northing;

        let false_easting = 500_000;
        let false_northing = 10_000_000;

        // WGS 84:  a = 6378137, b = 6356752.314245, f = 1/298.257223563;
        let a = utm.datum.a();
        let f = utm.datum.f();

        let k0 = 0.9996; // UTM scale on the central meridian

        x = x - false_easting;               // make x ± relative to central meridian
        y = if h == Hemisphere::S { y - false_northing } else { y }; // make y ± relative to equator

        // ---- from Karney 2011 Eq 15-22, 36:

        let e = f64::sqrt(f * (2.0 - f)); // eccentricity
        let n = f / (2.0 - f);        // 3rd flattening
        let n2 = n * n;
        let n3 = n * n2;
        let n4 = n * n3;
        let n5 = n * n4;
        let n6 = n * n5;

        let A = a / (1.0 + n) * (1.0 + 1.0 / 4.0 * n2 + 1.0 / 64.0 * n4 + 1.0 / 256.0 * n6); // 2πA is the circumference of a meridian

        let eta = x as f64 / (k0*A);
        let xi = y as f64 / (k0*A);

        let beta = [ 0.0, // note beta is one-based array (6th order Krüger expressions)
            1.0 / 2.0 * n - 2.0 / 3.0 * n2 + 37.0 / 96.0 * n3 - 1.0 / 360.0 * n4 - 81.0 / 512.0 * n5 + 96199.0 / 604800.0 * n6,
            1.0 / 48.0 * n2 + 1.0 / 15.0 * n3 - 437.0 / 1440.0 * n4 + 46.0 / 105.0 * n5 - 1118711.0 / 3870720.0 * n6,
            17.0 / 480.0 * n3 - 37.0 / 840.0 * n4 - 209.0 / 4480.0 * n5 + 5569.0 / 90720.0 * n6,
            4397.0 / 161280.0 * n4 - 11.0 / 504.0 * n5 - 830251.0 / 7257600.0 * n6,
            4583.0 / 161280.0 * n5 - 108847.0 / 3991680.0 * n6,
            20648693.0 / 638668800.0 * n6 ];

        let mut xi2 = xi;
        for j in 1..6 { xi2 -= beta[j] * f64::sin(2.0 * j as f64 * xi) * f64::cosh(2.0 * j as f64 * eta); }

        let mut eta2 = eta;
        for j in 1..6 { eta2 -= beta[j] * f64::cos(2.0 * j as f64* xi) * f64::sinh(2.0 * j as f64 * eta); }

        let sinheta2 = f64::sinh(eta2);
        let sinxi2 = f64::sin(xi2);
        let cosxi2 = f64::cos(xi2);

        let tau2 = sinxi2 / f64::sqrt(sinheta2 * sinheta2 + cosxi2 * cosxi2);

        let taui = tau2;
        loop {
            let sigmai = f64::sinh(e * f64::atanh(e * taui / f64::sqrt(1.0 + taui * taui)));
            let taui2 = taui * f64::sqrt(1.0 + sigmai * sigmai) - sigmai * f64::sqrt(1.0 + taui * taui);
            let deltataui = (tau2 - taui2) / f64::sqrt(1.0 + taui2 * taui2)
                * (1.0 + (1.0 - e * e) * taui * taui) / ((1.0 - e * e) * f64::sqrt(1.0 + taui * taui));
             taui += deltataui;
            if !(f64::abs(deltataui) > 1e-12) { break; } // using IEEE 754 deltataui -> 0 after 2-3 iterations
        }
        // note relatively large convergence test as deltataui toggles on ±1.12e-16 for eg 31 N 400000 5000000
        let tau = taui;

        let phi = f64::atan(tau);

        let lamda = f64::atan2(sinheta2, cosxi2);

        // ---- convergence: Karney 2011 Eq 26, 27

        let mut p = 1.0;
        for j in 1..6 { p -= 2.0 * j as f64 * beta[j] * f64::cos(2.0 * j as f64 * xi) * f64::cosh(2.0 * j as f64 * eta); }
        let mut q = 0.0;
        for j in 1..6 { q += 2.0 * j as f64 * beta[j] * f64::sin(2.0 * j as f64 * xi) * f64::sinh(2.0 * j as f64 * eta); }

        let gamma2 = f64::atan(f64::tan(xi2) * f64::tanh(eta2));
        let gamma3 = f64::atan2(q, p);

        let gamma = gamma2 + gamma3;

        // ---- scale: Karney 2011 Eq 28

        let sinphi = f64::sin(phi);
        let k2 = f64::sqrt(1.0 - e * e * sinphi * sinphi) * f64::sqrt(1.0 + tau * tau) * f64::sqrt(sinheta2 *sinheta2 + cosxi2 * cosxi2);
        let k3 = A / a / f64::sqrt(p * p + q * q);

        let k = k0 * k2 * k3;

        // ------------

        let lamda0: f64 = f64::to_radians(((z - 1) * 6 - 180 + 3) as f64); // longitude of central meridian
        lamda += lamda0; // move lamda from zonal to global coordinates

        // round to reasonable precision
        let to_precisionf = |x: f64, y: usize| -> f64 {
            let p = f64::powf(10.0, y as f64);
            f64::round(x * p) / p
        };

        // round to reasonable precision
        let lat = to_precisionf(phi.to_degrees(), 11); // nm precision (1nm = 10^-11°)
        let lon = to_precisionf(lamda.to_degrees(), 11); // (strictly lat rounding should be phi⋅cosphi!)
        let convergence = to_precisionf(gamma.to_degrees(), 9);
        let scale = to_precisionf(k, 12);

        LatLon {
            lat: lat,
            lon: lon,
            datum: utm.datum,
            convergence: Some(convergence),
            scale: Some(scale),
        }
    }
}
