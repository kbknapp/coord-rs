use std::str::FromStr;
use std::fmt;

use Errors;
use band::LatBand;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Hemisphere {
    N,
    S,
}

///////////////////////////////////
///////////// impls ///////////////
///////////////////////////////////

impl Default for Hemisphere {
    fn default() -> Self {
        Hemisphere::N
    }
}

impl From<char> for Hemisphere {
    fn from(c: char) -> Self {
        match c {
            'N' | 'n' => Hemisphere::N,
            'S' | 's' => Hemisphere::S,
            _ => panic!("Invalid hemisphere character {}", c),
        }
    }
}

impl From<f64> for Hemisphere {
    fn from(lat: f64) -> Self {
        if lat >= 0.0 {
            Hemisphere::N
        } else {
            Hemisphere::S
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

impl<'s> From<&'s str> for Hemisphere {
    fn from(s: &'s str) -> Self {
        Hemisphere::from(s.as_ref())
    }
}

// impl<S: Into<String>> From<S> for Hemisphere {
//     fn from(s: S) -> Self {
//         Hemisphere::from(&*s.into()).expect("Invalid Hemisphere character")
//     }
// }

impl From<LatBand> for Hemisphere {
    fn from(lb: LatBand) -> Self {
        use band::LatBand::{N, P, Q, R, S, T, U, V, W, X};
        match lb {
            N | P | Q | R | S | T | U | V |  W | X => Hemisphere::N,
            _ => Hemisphere::S,
        }
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
        match h { N => 'N', S => 'S', }
    }
}

impl fmt::Display for Hemisphere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c: char = (*self).into();
        writeln!(f, "{}", c)
    }
}
