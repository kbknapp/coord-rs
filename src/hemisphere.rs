use std::str::FromStr;

use Errors;

#[derive(Copy, Clone, Debug)]
pub enum Hemisphere {
    N,
    S,
}

///////////////////////////////////
///////////// impls ///////////////
///////////////////////////////////

impl From<char> for Hemisphere {
    fn from(c: char) -> Self {
        match c {
            'N' | 'n' => Hemisphere::N,
            'S' | 's' => Hemisphere::S,
            _ => panic!("Invalid hemisphere character {}", c),
        }
    }
}

impl From<u32> for Hemisphere {
    fn from(c: u32) -> Self {
        Hemisphere::from(c as u8)
    }
}

impl From<u8> for Hemisphere {
    fn from(c: u8) -> Self {
        Hemisphere::from(c as char)
    }
}

impl<S: AsRef<str>> From<S> for Hemisphere {
    fn from(s: S) -> Self {
        Hemisphere::from(s.as_ref()).expect("Invalid Hemisphere character")
    }
}

impl<S: Into<String>> From<S> for Hemisphere {
    fn from(s: S) -> Self {
        Hemisphere::from(&*s.into()).expect("Invalid Hemisphere character")
    }
}

impl FromStr for Hemisphere {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s.chars().nth(0).expect("Empty Hemisphere string for conversion");
        match c {
            'n' | 'N' => Ok(Hemisphere::N),
            's' | 'S' => Ok(Hemisphere::S),
            _ => Err(Errors::InvalidHemisphereChar(c))
        }
    }
}

impl From<Hemisphere> for char {
    fn from(h: Hemisphere) -> Self {
        use self::Hemisphere::{N, S};
        match r { N => 'N', S => 'S', }
    }
}
