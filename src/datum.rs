use std::str::FromStr;
use std::ascii::AsciiExt;

#[derive(Copy, Clone, Debug)]
pub enum Datum {
    Wgs84
}

impl Default for Datum {
    fn default() -> Self {
        Datum::Wgs84
    }
}

///////////////////////////////////
///////////// impls ///////////////
///////////////////////////////////

impl<S: AsRef<str>> From<S> for Datum {
    fn from(s: S) -> Self {
        Datum::from(s.as_ref()).expect("Invalid Datum string")
    }
}

impl<S: Into<String>> From<S> for Datum {
    fn from(s: S) -> Self {
        Datum::from(&*s.into()).expect("Invalid Datum string")
    }
}

impl FromStr for Hemisphere {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let d = s.to_ascii_uppercase();
        match d {
            "WGS84" => Ok(Datum::Wgs84),
            _ => Err(Errors::InvalidDatum(s.to_owned()))
        }
    }
}
