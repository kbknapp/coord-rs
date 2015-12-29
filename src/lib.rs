#![crate_type= "lib"]
#![cfg_attr(feature = "nightly", feature(plugin))]
#![cfg_attr(feature = "lints", plugin(clippy))]
#![cfg_attr(feature = "lints", deny(warnings))]
#![cfg_attr(not(any(feature = "lints", feature = "nightly")), deny(unstable_features))]
#![deny(//missing_docs,
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unused_import_braces,
        unused_qualifications)]

#[macro_use]
mod macros;
mod errors;
mod gzd;
mod utm;
mod latlon;
mod ascii;
mod mgrs;
mod accuracy;
mod parser;
mod datum;
mod hemisphere;
mod band;
mod col;
mod row;

pub use errors::Errors;
pub use band::LatBand;
pub use gzd::Gzd;
pub use utm::Utm;
pub use mgrs::Mgrs;
pub use accuracy::Accuracy;
pub use latlon::LatLon;

pub type Lat = f64;
impl From<LatBand> for f64 {
    fn from(band: LatBand) -> Self {
        ((band.index() - 10) * 8) as f64
    }
}
pub type Lon = f64;

/// UTM zones are grouped, and assigned to one of a group of 6 sets
const NUM_100K_SETS: usize = 6;

/// The column letters (for easting) of the lower left value, per set
// A=65, J=74, S=83
const SET_ORIGIN_COLUMN_LETTERS: [u8; 6] = [b'A', b'J', b'S', b'A', b'J', b'S'];

// The row letters (for northing) of the lower left value, per set
// A=65, F=70
const SET_ORIGIN_ROW_LETTERS: [u8; 6]  = [b'A', b'F', b'A', b'F', b'A', b'F'];

fn get_100k_set_for_zone(i: usize) -> usize {
    /*!
    Given a UTM zone number, figure out the MGRS 100K set it is in.

    ### Params
     * **i**: A UTM zone number.
    ### Return
     * The 100k set the UTM zone is in.
    */
    let mut set = i % NUM_100K_SETS;
    if set == 0 {
        set = NUM_100K_SETS;
    }
    set
}
