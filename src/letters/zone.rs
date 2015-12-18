use std::convert::From;
use std::str;
use std::error::Error;

use Lat;
use Errors;

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub enum ZoneLetter {
    C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V, W, X
}

impl ZoneLetter {
    pub fn from_lat(l: Lat) -> Option<Self> {
        /*!
        Calculates the MGRS letter designator for the given latitude.

        ### Params
         * **l**: The latitude in WGS84 to get the letter designator for.
        ### Return
         * **Some** The `ZoneLetter` designator.
         * **None** If no letter exists for the given lattiude
        */

        use self::ZoneLetter::{C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V, W, X};
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

        use ZoneLetter::{C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V, W, X};

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
        Err(Errors::InvalidZoneLetter((*self).into()))
    }
}

impl From<Lat> for ZoneLetter {
    fn from(lat: Lat) -> Self {
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

        return match ZoneLetter::from_lat(lat) {
            Some(z) => z,
            None => panic!("No Grid Zone Letter for Lattitude: {}", lat),
        }
    }
}

impl ::std::str::FromStr for ZoneLetter {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ZoneLetter::{C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V, W, X};
        // Check first char, or fail (Z doesn't exist)
        let z = s.as_bytes()[0];
        match z {
            b'C' | b'c' => Ok(C), b'D' | b'd' => Ok(D), b'E' | b'e' => Ok(E), b'F' | b'f' => Ok(F),
            b'G' | b'g' => Ok(G), b'H' | b'h' => Ok(H), b'J' | b'j' => Ok(J), b'K' | b'k' => Ok(K),
            b'L' | b'l' => Ok(L), b'M' | b'm' => Ok(M), b'N' | b'n' => Ok(N), b'P' | b'p' => Ok(P),
            b'Q' | b'q' => Ok(Q), b'R' | b'r' => Ok(R), b'S' | b's' => Ok(S), b'T' | b't' => Ok(T),
            b'U' | b'u' => Ok(U), b'V' | b'v' => Ok(V), b'W' | b'w' => Ok(W), b'X' | b'x' => Ok(X),
            _ => Err(Errors::InvalidZoneLetter(z as char))
        }
    }
}

impl From<char> for ZoneLetter {
    fn from(c: char) -> Self {
        let b = &[c as u8];
        let s = unsafe { str::from_utf8_unchecked(b) };
        return match s.parse() {
            Ok(z) => z,
            Err(e) => panic!(e.description().to_owned())
        };
    }
}

impl From<ZoneLetter> for char {
    fn from(r: ZoneLetter) -> Self {
        use self::ZoneLetter::{C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V, W, X};
        match r {
            C => 'C', D => 'D', E => 'E', F => 'F', G => 'G', H => 'H', J => 'J', K => 'K',
            L => 'L', M => 'M', N => 'N', P => 'P', Q => 'Q', R => 'R', S => 'S', T => 'T',
            U => 'U', V => 'V', W => 'W', X => 'X'
        }
    }
}

impl Default for ZoneLetter {
    fn default() -> Self {
        // Shuold we use an "invalid" letter?
        ZoneLetter::C
    }
}
