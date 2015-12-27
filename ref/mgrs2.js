/**
 * Converts UTM zone/easting/northing coordinate to latitude/longitude
 *
 * @param   {Utm}     utmCoord - UTM coordinate to be converted to latitude/longitude.
 * @returns {LatLon} Latitude/longitude of supplied grid reference.
 *
 * @example
 *   var grid = new Utm(31, 'N', 448251.795, 5411932.678);
 *   var latlong = grid.toLatLonE(); // latlong.toString(): 48°51′29.52″N, 002°17′40.20″E
 */
Utm.toLatLonE() {
    var z = this.zone;
    var h = this.hemisphere;
    var x = this.easting;
    var y = this.northing;

    if (isNaN(z) || isNaN(x) || isNaN(y)) throw new Error('Invalid coordinate');

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
};


/**
 * Parses string representation of UTM coordinate.
 *
 * A UTM coordinate comprises (space-separated)
 *  - zone
 *  - hemisphere
 *  - easting
 *  - northing.
 *
 * @param   {string} utmCoord - UTM coordinate (WGS 84).
 * @param   {Datum}  [datum=WGS84] - Datum coordinate is defined in (default WGS 84).
 * @returns {Utm}
 * @throws  Error Invalid UTM coordinate
 *
 * @example
 *   var utmCoord = Utm.parse('31 N 448251 5411932');
 *   // utmCoord: {zone: 31, hemisphere: 'N', easting: 448251, northing: 5411932 }
 */
Utm.parse = function(utmCoord, datum) {
    if (datum === undefined) datum = LatLon.datum.WGS84; // default if not supplied

    // match separate elements (separated by whitespace)
    utmCoord = utmCoord.trim().match(/\S+/g);

    if (utmCoord==null || utmCoord.length!=4) throw new Error('Invalid UTM coordinate');

    var zone = utmCoord[0], hemisphere = utmCoord[1], easting = utmCoord[2], northing = utmCoord[3];

    return new Utm(zone, hemisphere, easting, northing, datum);
};

/**
 * Parses string representation of MGRS grid reference.
 *
 * An MGRS grid reference comprises (space-separated)
 *  - grid zone designator (GZD)
 *  - 100km grid square letter-pair
 *  - easting
 *  - northing.
 *
 * @param  {string} mgrsGridRef - String representation of MGRS grid reference.
 * @returns {Mgrs} Mgrs grid reference object.
 *
 * @example
 *   var mgrsRef = Mgrs.parse('31U DQ 48251 11932');
 *   var mgrsRef = Mgrs.parse('31UDQ4825111932');
 *   //  mgrsRef: { zone:31, band:'U', e100k:'D', n100k:'Q', easting:48251, northing:11932 }
 */
Mgrs.parse(mgrsGridRef) {
    mgrsGridRef = mgrsGridRef.trim();

    // check for military-style grid reference with no separators
    if (!mgrsGridRef.match(/\s/)) {
        var en = mgrsGridRef.slice(5); // get easting/northing following zone/band/100ksq
        en = en.slice(0, en.length/2)+' '+en.slice(-en.length/2); // separate easting/northing
        mgrsGridRef = mgrsGridRef.slice(0, 3)+' '+mgrsGridRef.slice(3, 5)+' '+en; // insert spaces
    }

    // match separate elements (separated by whitespace)
    mgrsGridRef = mgrsGridRef.match(/\S+/g);

    if (mgrsGridRef==null || mgrsGridRef.length!=4) throw new Error('Invalid MGRS grid reference');

    // split gzd into zone/band
    var gzd = mgrsGridRef[0];
    var zone = gzd.slice(0, 2);
    var band = gzd.slice(2, 3);

    // split 100km letter-pair into e/n
    var en100k = mgrsGridRef[1];
    var e100k = en100k.slice(0, 1);
    var n100k = en100k.slice(1, 2);

    var e = mgrsGridRef[2], n = mgrsGridRef[3];

    // standardise to 10-digit refs - ie metres) (but only if < 10-digit refs, to allow decimals)
    e = e.length>=5 ?  e : (e+'00000').slice(0, 5);
    n = n.length>=5 ?  n : (n+'00000').slice(0, 5);

    return new Mgrs(zone, band, e100k, n100k, e, n);
};
