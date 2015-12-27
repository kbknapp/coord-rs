use std::error::Error;
use std::fmt;
use Lat;

#[derive(Debug, Clone)]
pub enum Errors {
    InvalidZoneLetter(char),
    InvalidRowLetter(char),
    InvalidColLetter(char),
    InvalidNorthingChar(char),
    InvalidEastingChar(char),
    InvalidLatitude(Lat),
    InvalidLatitudeBand(char),
    InvalidDatum(String),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.description())
    }
}

impl Error for Errors {
    fn description(&self) -> &str {
        match *self {
            Errors::InvalidZoneLetter(..)   => "invalid zone letter",
            Errors::InvalidColLetter(..)   => "invalid column letter",
            Errors::InvalidRowLetter(..)   => "invalid row letter",
            Errors::InvalidNorthingChar(..) => "MGRS point given invalid northing",
            Errors::InvalidEastingChar(..) => "MGRS point given invalid easting",
            Errors::InvalidLatitude(..) => "latitude outside UTM limits",
            Errors::InvalidLatitudeBand(..) => "invalid Latitude band letter",
            Errors::InvalidDatum(..) => "invalid map datum was supplied",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
