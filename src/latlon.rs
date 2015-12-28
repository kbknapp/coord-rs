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
        /*!
        Converts UTM zone/easting/northing coordinate to latitude/longitude

        @param   {Utm}     utmCoord - UTM coordinate to be converted to latitude/longitude.
        @returns {LatLon} Latitude/longitude of supplied grid reference.

        @example
          var grid = new Utm(31, 'N', 448251.795, 5411932.678);
          var latlong = grid.toLatLonE(); // latlong.toString(): 48°51′29.52″N, 002°17′40.20″E
        */
        var z = this.zone;
        var h = this.hemisphere;
        var x = this.easting;
        var y = this.northing;

        var falseEasting = 500e3, falseNorthing = 10000e3;

        var a = this.datum.ellipsoid.a, f = this.datum.ellipsoid.f;
        // WGS 84:  a = 6378137, b = 6356752.314245, f = 1/298.257223563;

        var k0 = 0.9996; // UTM scale on the central meridian

        x = x - falseEasting;               // make x ± relative to central meridian
        y = h=='S' ? y - falseNorthing : y; // make y ± relative to equator

        // ---- from Karney 2011 Eq 15-22, 36:

        var e = Math.sqrt(f*(2-f)); // eccentricity
        var n = f / (2 - f);        // 3rd flattening
        var n2 = n*n, n3 = n*n2, n4 = n*n3, n5 = n*n4, n6 = n*n5;

        var A = a/(1+n) * (1 + 1/4*n2 + 1/64*n4 + 1/256*n6); // 2πA is the circumference of a meridian

        var η = x / (k0*A);
        var ξ = y / (k0*A);

        var β = [ 0, // note β is one-based array (6th order Krüger expressions)
            1/2*n - 2/3*n2 + 37/96*n3 - 1/360*n4 - 81/512*n5 + 96199/604800*n6,
            1/48*n2 + 1/15*n3 - 437/1440*n4 + 46/105*n5 - 1118711/3870720*n6,
            17/480*n3 - 37/840*n4 - 209/4480*n5 + 5569/90720*n6,
            4397/161280*n4 - 11/504*n5 - 830251/7257600*n6,
            4583/161280*n5 - 108847/3991680*n6,
            20648693/638668800*n6 ];

        var ξʹ = ξ;
        for (var j=1; j<=6; j++) ξʹ -= β[j] * Math.sin(2*j*ξ) * Math.cosh(2*j*η);

        var ηʹ = η;
        for (var j=1; j<=6; j++) ηʹ -= β[j] * Math.cos(2*j*ξ) * Math.sinh(2*j*η);

        var sinhηʹ = Math.sinh(ηʹ);
        var sinξʹ = Math.sin(ξʹ), cosξʹ = Math.cos(ξʹ);

        var τʹ = sinξʹ / Math.sqrt(sinhηʹ*sinhηʹ + cosξʹ*cosξʹ);

        var τi = τʹ;
        do {
            var σi = Math.sinh(e*Math.atanh(e*τi/Math.sqrt(1+τi*τi)));
            var τiʹ = τi * Math.sqrt(1+σi*σi) - σi * Math.sqrt(1+τi*τi);
            var δτi = (τʹ - τiʹ)/Math.sqrt(1+τiʹ*τiʹ)
                * (1 + (1-e*e)*τi*τi) / ((1-e*e)*Math.sqrt(1+τi*τi));
             τi += δτi;
        } while (Math.abs(δτi) > 1e-12); // using IEEE 754 δτi -> 0 after 2-3 iterations
        // note relatively large convergence test as δτi toggles on ±1.12e-16 for eg 31 N 400000 5000000
        var τ = τi;

        var φ = Math.atan(τ);

        var λ = Math.atan2(sinhηʹ, cosξʹ);

        // ---- convergence: Karney 2011 Eq 26, 27

        var p = 1;
        for (var j=1; j<=6; j++) p -= 2*j*β[j] * Math.cos(2*j*ξ) * Math.cosh(2*j*η);
        var q = 0;
        for (var j=1; j<=6; j++) q += 2*j*β[j] * Math.sin(2*j*ξ) * Math.sinh(2*j*η);

        var γʹ = Math.atan(Math.tan(ξʹ) * Math.tanh(ηʹ));
        var γʺ = Math.atan2(q, p);

        var γ = γʹ + γʺ;

        // ---- scale: Karney 2011 Eq 28

        var sinφ = Math.sin(φ);
        var kʹ = Math.sqrt(1 - e*e*sinφ*sinφ) * Math.sqrt(1 + τ*τ) * Math.sqrt(sinhηʹ*sinhηʹ + cosξʹ*cosξʹ);
        var kʺ = A / a / Math.sqrt(p*p + q*q);

        var k = k0 * kʹ * kʺ;

        // ------------

        var λ0 = ((z-1)*6 - 180 + 3).toRadians(); // longitude of central meridian
        λ += λ0; // move λ from zonal to global coordinates

        // round to reasonable precision
        var lat = Number(φ.toDegrees().toFixed(11)); // nm precision (1nm = 10^-11°)
        var lon = Number(λ.toDegrees().toFixed(11)); // (strictly lat rounding should be φ⋅cosφ!)
        var convergence = Number(γ.toDegrees().toFixed(9));
        var scale = Number(k.toFixed(12));

        var latLong = new LatLon(lat, lon, this.datum);
        // ... and add the convergence and scale into the LatLon object ... wonderful JavaScript!
        latLong.convergence = convergence;
        latLong.scale = scale;

        return latLong;
    }
}
