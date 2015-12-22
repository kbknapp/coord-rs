use std::str::FromStr;
use std::convert::From;

use ascii;
use Errors;

use SET_ORIGIN_ROW_LETTERS;

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub enum RowLetter {
    A, B, C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V
}

impl RowLetter {
    pub fn get_northing_with_set(&self, set: u8) -> Result<f64, Errors> {
        /*!
        Given the second letter from a two-letter MGRS 100k zone, and given the
        MGRS table set for the zone number, figure out the northing value that
        should be added to the other, secondary northing value. You have to
        remember that Northings are determined from the equator, and the vertical
        cycle of letters mean a 2000000 additional northing meters. This happens
        approx. every 18 degrees of latitude. This method does *NOT* count any
        additional northings. You have to figure out how many 2000000 meters need
        to be added for the zone letter of the MGRS coordinate.

        ### Params
         * **n**: Second letter of the MGRS 100k zone
         * **set**: The MGRS table set number, which is dependent on the UTM zone number.
        ### Return
         * The northing value for the given letter and set.
        */
        let c: char = self.into();
        if c as u8 > ascii::V {
            return Err(Errors::InvalidNorthingChar(self.into()));
        }

        // rowOrigin is the letter at the origin of the set for the
        // column
        let mut cur_row = SET_ORIGIN_ROW_LETTERS[(set - 1) as usize];
        let mut northing_val: f64 = 0.0;
        let mut rewind_marker = false;

        let c: char = (*self).into();
        while cur_row != c as u8 {
            cur_row += 1;
            if cur_row == ascii::I {
                cur_row += 1;
            }
            if cur_row == ascii::O {
                cur_row += 1;
            }
            // fixing a bug making whole application hang in this loop
            // when 'n' is a wrong character
            if cur_row > ascii::V {
                if rewind_marker { // making sure that this loop ends
                    return Err(Errors::InvalidNorthingChar(self.into()));
                }
                cur_row = ascii::A;
                rewind_marker = true;
            }
            northing_val += 100000.0;
        }

        Ok(northing_val)
    }
}

impl FromStr for RowLetter {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::RowLetter::{A, B, C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V};
        // Check first char, or fail (Z doesn't exist)
        let z = s.as_bytes()[0];
        match z {
            b'A' | b'a' => Ok(A), b'B' | b'b' => Ok(B),
            b'C' | b'c' => Ok(C), b'D' | b'd' => Ok(D), b'E' | b'e' => Ok(E), b'F' | b'f' => Ok(F),
            b'G' | b'g' => Ok(G), b'H' | b'h' => Ok(H), b'J' | b'j' => Ok(J), b'K' | b'k' => Ok(K),
            b'L' | b'l' => Ok(L), b'M' | b'm' => Ok(M), b'N' | b'n' => Ok(N), b'P' | b'p' => Ok(P),
            b'Q' | b'q' => Ok(Q), b'R' | b'r' => Ok(R), b'S' | b's' => Ok(S), b'T' | b't' => Ok(T),
            b'U' | b'u' => Ok(U), b'V' | b'v' => Ok(V),
            _ => Err(Errors::InvalidRowLetter(z as char))
        }
    }
}

impl From<u32> for RowLetter {
    fn from(c: u32) -> Self {
        RowLetter::from(c as u8 as char)
    }
}

impl From<RowLetter> for char {
    fn from(r: RowLetter) -> Self {
        use self::RowLetter::{A, B, C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V};
        match r {
            A => 'A', B => 'B', C => 'C', D => 'D', E => 'E', F => 'F', G => 'G', H => 'H',
            J => 'J', K => 'K', L => 'L', M => 'M', N => 'N', P => 'P', Q => 'Q', R => 'R',
            S => 'S', T => 'T', U => 'U', V => 'V'
        }
    }
}

impl<'a> From<&'a RowLetter> for char {
    fn from(r: &'a RowLetter) -> Self {
        use self::RowLetter::{A, B, C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V};
        match *r {
            A => 'A', B => 'B', C => 'C', D => 'D', E => 'E', F => 'F', G => 'G', H => 'H',
            J => 'J', K => 'K', L => 'L', M => 'M', N => 'N', P => 'P', Q => 'Q', R => 'R',
            S => 'S', T => 'T', U => 'U', V => 'V'
        }
    }
}

impl From<char> for RowLetter {
    fn from(c: char) -> Self {
        RowLetter::from(c as u32)
    }
}

impl Default for RowLetter {
    fn default() -> Self {
        RowLetter::A
    }
}

#[cfg(test)]
mod test {
    use super::RowLetter;

    #[test]
    fn from_char() {
        let a = 'a';
        let cl = RowLetter::from(a);
        assert_eq!(cl, RowLetter::A);

        let a = 'A';
        let cl = RowLetter::from(a);
        assert_eq!(cl, RowLetter::A);
    }

    #[test]
    fn from_u32() {
        let c: u32 = 99;
        let cl = RowLetter::from(c);
        assert_eq!(cl, RowLetter::C);

        let c: u32 = 67;
        let cl = RowLetter::from(c);
        assert_eq!(cl, RowLetter::C);
    }

    #[test]
    fn from_str() {
        let c = "c";
        let cl: RowLetter = c.parse();
        assert_eq!(cl, Ok(RowLetter::C));

        let c = "C";
        let cl: RowLetter = c.parse();
        assert_eq!(cl, Ok(RowLetter::C));
    }

    #[test]
    fn to_char() {
        let cl = RowLetter::C;
        let c = char::from(cl);
        assert_eq!(c, 'C');
    }
}
