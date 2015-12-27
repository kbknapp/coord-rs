use std::convert::From;
use std::str::{self, FromStr};
use std::error::Error;

use Errors;

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
/// 8° latitudinal band (C..X covering 80°S..84°N) **note:** X is repeated for 80-84°N
pub enum LatBand {
    C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V, W, X
}

impl LatBand {
    pub fn from_lat(l: f64) -> Option<Self> {
        /*!
        Calculates the MGRS letter designator for the given latitude.

        ### Params
         * **l**: The latitude in WGS84 datum to get the zone band letter for.
        ### Return
         * **Some** The `BandLetter` designator.
         * **None** If no letter exists for the given lattiude
        */

        use self::LatBand::{C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V, W, X};
        match l as isize {
            l if ((84 >= l) && (l >= 72)) => Some(X),
            l if ((72 > l)  && (l >= 64)) => Some(W),
            l if ((64 > l)  && (l >= 56)) => Some(V),
            l if ((56 > l)  && (l >= 48)) => Some(U),
            l if ((48 > l)  && (l >= 40)) => Some(T),
            l if ((40 > l)  && (l >= 32)) => Some(S),
            l if ((32 > l)  && (l >= 24)) => Some(R),
            l if ((24 > l)  && (l >= 16)) => Some(Q),
            l if ((16 > l)  && (l >= 8))  => Some(P),
            l if ((8 > l)   && (l >= 0))  => Some(N),
            l if ((0 > l)   && (l >= -8)) => Some(M),
            l if ((-8 > l)  && (l >= -16)) => Some(L),
            l if ((-16 > l) && (l >= -24)) => Some(K),
            l if ((-24 > l) && (l >= -32)) => Some(J),
            l if ((-32 > l) && (l >= -40)) => Some(H),
            l if ((-40 > l) && (l >= -48)) => Some(G),
            l if ((-48 > l) && (l >= -56)) => Some(F),
            l if ((-56 > l) && (l >= -64)) => Some(E),
            l if ((-64 > l) && (l >= -72)) => Some(D),
            l if ((-72 > l) && (l >= -80)) => Some(C),
            _ => None,
        }

    }

    // fn alt_from_lat(l: f64) -> Self {
    //     LatBand::index(f64::floor((l / 8.0) + 10.0))
    // }

    pub fn get_min_northing(&self) -> Result<f64, Errors> {
        /*!
        Returns the minimum northing value of a MGRS zone.

        Ported from proj4js/mgrs:getMinNorthing which was ported from Geotrans' c lattitude_Band_Value
        structure table.

        ### Params
         * **zone_letter**: The MGRS zone to get the min northing for.

        ### Return
         * **Ok**: The minimum northing for that zone letter
         * **Err**: Returns `Errors::InvalidZoneLetter` if the `zone_letter` isn't valid
        */

        use self::LatBand::{C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V, W, X};

        let northing = match *self {
            C => 1100000.0,
            D => 2000000.0,
            E => 2800000.0,
            F => 3700000.0,
            G => 4600000.0,
            H => 5500000.0,
            J => 6400000.0,
            K => 7300000.0,
            L => 8200000.0,
            M => 9100000.0,
            N => 0.0,
            P => 800000.0,
            Q => 1700000.0,
            R => 2600000.0,
            S => 3500000.0,
            T => 4400000.0,
            U => 5300000.0,
            V => 6200000.0,
            W => 7000000.0,
            X => 7900000.0,
            // _   => -1.0,
        };
        if northing >= 0.0 {
            return Ok(northing);
        }
        Err(Errors::InvalidLatitudeBand(self.as_char()))
    }

    pub fn index(&self) -> usize {
        match *self {
            C => 0, D => 1, E => 2, F => 3, G => 4, H => 5, J => 6, K => 7, L => 8, M => 9,
            N => 10, P => 11, Q => 12, R => 13, S => 14, T => 15, U => 16, V => 17, W => 18,
            X => 19,
        }
    }

    pub fn as_char(&self) -> char {
        use self::LatBand::{C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V, W, X};
        match *self {
            C => 'C', D => 'D', E => 'E', F => 'F', G => 'G', H => 'H',
            J => 'J', K => 'K', L => 'L', M => 'L', N => 'N', P => 'P',
            Q => 'Q', R => 'R', S => 'S', T => 'T', U => 'U', V => 'V',
            W => 'W', X => 'X'
        }
    }
}

///////////////////////////////////////////////////
//////////////// impls ////////////////////////////
///////////////////////////////////////////////////

impl From<f64> for LatBand {
    fn from(lat: f64) -> Self {
        /*!
        Calculates the MGRS letter designator for the given latitude.

        ### Params
         * **lat**: The latitude in WGS84 to get the letter designator for.
        ### Return
         * The `ZoneLetter` designator.

        # Panics

        This fuction will panic if a lattitude without a given Grid Zone Letter is presented. If
        this is not the desired behavior, prefer the `ZoneLetter::letter_for_lat` instead.
        */

        return match LatBand::from_lat(lat) {
            Some(z) => z,
            None => panic!("No Grid Zone Letter for Lattitude: {}", lat),
        }
    }
}

impl From<char> for LatBand {
    fn from(c: char) -> Self {
        use self::LatBand::{C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V, W, X};
        match c {
            'C' | 'c' => C, 'D' | 'd' => D, 'E' | 'e' => E, 'F' | 'f' => F, 'G' | 'g' => G,
            'H' | 'h' => H, 'J' | 'j' => J, 'K' | 'k' => K, 'L' | 'l' => L, 'M' | 'm' => M,
            'N' | 'n' => N, 'P' | 'p' => P, 'Q' | 'q' => Q, 'R' | 'r' => R, 'S' | 's' => S,
            'T' | 't' => T, 'U' | 'u' => U, 'V' | 'v' => V, 'W' | 'w' => W, 'X' | 'x' => X,
            _ => panic!("invalid latitude band letter {}", c), 
        }
    }
}

impl From<u32> for LatBand {
    fn from(c: u32) -> Self {
        LatBand::from(c as u8)
    }
}

impl From<u8> for LatBand {
    fn from(c: u8) -> Self {
        LatBand::from(c as char)
    }
}

impl<'s> From<&'s str> for LatBand {
    fn from(s: &'s str) -> Self {
        LatBand::from(s)
    }
}

// impl<S: Into<String>> From<S> for LatBand {
//     fn from(s: S) -> Self {
//         LatBand::from(&*s.into()).expect("Invalid latitudinal band character")
//     }
// }

impl From<LatBand> for char {
    fn from(r: LatBand) -> Self {
        use self::LatBand::{C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V, W, X};
        match r {
            C => 'C', D => 'D', E => 'E', F => 'F', G => 'G', H => 'H', J => 'J', K => 'K',
            L => 'L', M => 'M', N => 'N', P => 'P', Q => 'Q', R => 'R', S => 'S', T => 'T',
            U => 'U', V => 'V', W => 'W', X => 'X'
        }
    }
}

impl FromStr for LatBand {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::LatBand::{C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V, W, X};
        // Check first char, or fail (Z doesn't exist)
        let z = s.as_bytes()[0];
        match z {
            b'C' | b'c' => Ok(C), b'D' | b'd' => Ok(D), b'E' | b'e' => Ok(E), b'F' | b'f' => Ok(F),
            b'G' | b'g' => Ok(G), b'H' | b'h' => Ok(H), b'J' | b'j' => Ok(J), b'K' | b'k' => Ok(K),
            b'L' | b'l' => Ok(L), b'M' | b'm' => Ok(M), b'N' | b'n' => Ok(N), b'P' | b'p' => Ok(P),
            b'Q' | b'q' => Ok(Q), b'R' | b'r' => Ok(R), b'S' | b's' => Ok(S), b'T' | b't' => Ok(T),
            b'U' | b'u' => Ok(U), b'V' | b'v' => Ok(V), b'W' | b'w' => Ok(W), b'X' | b'x' => Ok(X),
            _ => Err(Errors::InvalidLatitudeBand(z as char))
        }
    }
}

impl Default for LatBand {
    fn default() -> Self {
        // Shuold we use an "invalid" letter?
        LatBand::C
    }
}
