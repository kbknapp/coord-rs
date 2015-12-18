use std::error::Error;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Errors {
    InvalidZoneLetter(char),
    InvalidRowLetter(char),
    InvalidColLetter(char),
    InvalidNorthingChar(char),
    InvalidEastingChar(char),
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
            Errors::InvalidNorthingChar(..) => "MGRS Point given invalid Northing",
            Errors::InvalidEastingChar(..) => "MGRS Point given invalid Easting",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
