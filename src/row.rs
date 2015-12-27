use std::str::FromStr;
use std::convert::From;

use ascii;
use Errors;

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
/// 100km grid square row letters
///
/// Repeats every other zone with sets: 'ABCDEFGHJKLMNPQRSTUV', 'FGHJKLMNPQRSTUVABCDE'
pub enum RowLetter {
    A, B, C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V
}

impl RowLetter {
    pub fn get_northing_with_set(&self, zone: u8) -> usize {
        // get easting specified by e100k
        self.index_from_set((zone-1)%2) * 100000
    }

    pub fn index_for_northing(northing: i32) -> u8 {
        (f64::floor(northing / 100000) as u8) % 20
    }

    pub fn letter_at(zone: u8, index: u8) -> Self {
        RowLetter::from_set_and_index((zone - 1) % 2, index)
    }

    pub fn from_zone_and_northing(zone: u8, northing: i32) -> Self {
        RowLetter::letter_at(zone, RowLetter::index_for_northing(northing))
    }

    fn index_from_set(&self, set: u8) -> usize {
        use self::RowLetter::{A, B, C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V};
        match set {
            0 => match *self {
                A => 0, B => 1, C => 2, D => 3, E => 4, F => 5, G => 6, H => 7, J => 8, K => 9,
                L => 10, M => 11, N => 12, P => 13, Q => 14, R => 15, S => 16, T => 17, U => 18,
                V => 19,
                _ => panic!("Invalid n100k letter for set 1")
            },
            1 => match *self {
                F => 0, G => 1, H => 2, J => 3, K => 4, L => 5, M => 6, N => 7, P => 8, Q => 9,
                R => 10, S => 11, T => 12, U => 13, V => 14, A => 15, B => 16, C => 17, D => 18,
                E => 19,
                _ => panic!("Invalid n100k letter for set 2")
            },
            _ => panic!("Invalid n100k set"),
        }
    }

    fn from_set_and_index(set: u8, index: u8) -> Self {
        use self::RowLetter::{A, B, C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V};
        match set {
            0 => match index {
                0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G, 7 => H, 8 => J, 9 => K,
                10 => L, 11 => M, 12 => N, 13 => P, 14 => Q, 15 => R, 16 => S, 17 => T, 18 => U,
                19 => V,
                _ => panic!("Invalid n100k letter for set 1")
            },
            1 => match index {
                0 => F, 1 => G, 2 => H, 3 => J, 4 => K, 5 => L, 6 => M, 7 => N, 8 => P, 9 => Q,
                10 => R, 11 => S, 12 => T, 13 => U, 14 => V, 15 => A, 16 => B, 17 => C, 18 => D,
                19 => E,
                _ => panic!("Invalid n100k letter for set 2")
            },
            _ => panic!("Invalid n100k set"),
        }
    }
}

impl FromStr for RowLetter {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::RowLetter::{A, B, C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V};
        // Check first char, or fail (Z doesn't exist)
        let z = s.as_bytes()[0];
        match z {
            b'A' | b'a' => Ok(A), b'B' | b'b' => Ok(B), b'C' | b'c' => Ok(C), b'D' | b'd' => Ok(D),
            b'E' | b'e' => Ok(E), b'F' | b'f' => Ok(F), b'G' | b'g' => Ok(G), b'H' | b'h' => Ok(H),
            b'J' | b'j' => Ok(J), b'K' | b'k' => Ok(K), b'L' | b'l' => Ok(L), b'M' | b'm' => Ok(M),
            b'N' | b'n' => Ok(N), b'P' | b'p' => Ok(P), b'Q' | b'q' => Ok(Q), b'R' | b'r' => Ok(R),
            b'S' | b's' => Ok(S), b'T' | b't' => Ok(T), b'U' | b'u' => Ok(U), b'V' | b'v' => Ok(V),
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
