use ascii;
use Errors;

use SET_ORIGIN_COLUMN_LETTERS;

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub enum ColLetter {
    A, B, C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V, W, X, Y, Z
}

impl ColLetter {
    pub fn get_easting_with_set(&self, set: u8) -> Result<f64, Errors> {
        /*!
        Given the first letter from a two-letter MGRS 100k zone, and given the
        MGRS table set for the zone number, figure out the easting value that
        should be added to the other, secondary easting value.

        ### Params
         * **e**: The first letter from a two-letter MGRS 100´k zone.
         * **set The MGRS table set for the zone number.
        ### Return
         * The easting value for the given letter and set.
        */
        // cur_col is the letter at the origin of the set for the column
        let mut cur_col = SET_ORIGIN_COLUMN_LETTERS[(set - 1) as usize];
        let mut easting_val: f64 = 100000.0;
        let mut rewind_marker = false;

        let c: char = (*self).into();
        while cur_col != c as u8 {
            cur_col += 1;
            if cur_col == ascii::I {
                cur_col += 1;
            }
            if cur_col == ascii::O {
                cur_col += 1;
            }
            if cur_col > ascii::Z {
                if rewind_marker {
                    return Err(Errors::InvalidEastingChar((*self).into()));
                }
                cur_col = ascii::A;
                rewind_marker = true;
            }
            easting_val += 100000.0;
        }

        Ok(easting_val)
    }
}

impl ::std::str::FromStr for ColLetter {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::ColLetter::{A, B, C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V, W, X, Y, Z};
        let z = s.as_bytes()[0];
        match z {
            b'A' | b'a' => Ok(A), b'B' | b'b' => Ok(B),
            b'C' | b'c' => Ok(C), b'D' | b'd' => Ok(D), b'E' | b'e' => Ok(E), b'F' | b'f' => Ok(F),
            b'G' | b'g' => Ok(G), b'H' | b'h' => Ok(H), b'J' | b'j' => Ok(J), b'K' | b'k' => Ok(K),
            b'L' | b'l' => Ok(L), b'M' | b'm' => Ok(M), b'N' | b'n' => Ok(N), b'P' | b'p' => Ok(P),
            b'Q' | b'q' => Ok(Q), b'R' | b'r' => Ok(R), b'S' | b's' => Ok(S), b'T' | b't' => Ok(T),
            b'U' | b'u' => Ok(U), b'V' | b'v' => Ok(V), b'W' | b'w' => Ok(W), b'X' | b'x' => Ok(X),
            b'Y' | b'y' => Ok(Y), b'Z' | b'z' => Ok(Z),
            _ => Err(Errors::InvalidColLetter(z as char))
        }
    }
}

impl From<ColLetter> for char {
    fn from(c: ColLetter) -> Self {
        use self::ColLetter::{A, B, C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V, W, X, Y, Z};
        match c {
            A => 'A', B => 'B', C => 'C', D => 'D', E => 'E', F => 'F', G => 'G', H => 'H',
            J => 'J', K => 'K', L => 'L', M => 'M', N => 'N', P => 'P', Q => 'Q', R => 'R',
            S => 'S', T => 'T', U => 'U', V => 'V', W => 'W', X => 'X', Y => 'Y', Z => 'Z',
        }
    }
}

impl From<u32> for ColLetter {
    fn from(c: u32) -> Self {
        ColLetter::from(c as u8 as char)
    }
}

impl From<char> for ColLetter {
    fn from(c: char) -> Self {
        ColLetter::from(c as u32)
    }
}

impl Default for ColLetter {
    fn default() -> Self {
        ColLetter::A
    }
}

#[cfg(test)]
mod test {
    use super::ColLetter;
    
    #[test]
    fn from_char() {
        let a = 'a';
        let cl = ColLetter::from(a);
        assert_eq!(cl, ColLetter::A);

        let a = 'A';
        let cl = ColLetter::from(a);
        assert_eq!(cl, ColLetter::A);
    }

    #[test]
    fn from_u32() {
        let c: u32 = 99;
        let cl = ColLetter::from(c);
        assert_eq!(cl, ColLetter::C);

        let c: u32 = 67;
        let cl = ColLetter::from(c);
        assert_eq!(cl, ColLetter::C);
    }

    #[test]
    fn from_str() {
        let c = "c";
        let cl: ColLetter = c.parse();
        assert_eq!(cl, Ok(ColLetter::C));

        let c = "C";
        let cl: ColLetter = c.parse();
        assert_eq!(cl, Ok(ColLetter::C));
    }

    #[test]
    fn to_char() {
        let cl = ColLetter::C;
        let c = char::from(cl);
        assert_eq!(c, 'C');
    }
}
