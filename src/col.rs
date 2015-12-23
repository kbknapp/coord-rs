use ascii;
use Errors;

use SET_ORIGIN_COLUMN_LETTERS;

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
/// 100km grid square column letters
///
/// Repeats every third zone with sets: 'ABCDEFGH', 'JKLMNPQR', 'STUVWXYZ'
pub enum ColLetter {
    A, B, C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V, W, X, Y, Z
}


impl ColLetter {
    pub fn as_meters_from_zone(&self, zone: u8) -> usize {
        // get easting specified by e100k
        self.index_from_set((zone-1)%3) * 100000
    }

    fn index_from_set(&self, set: u8) -> usize {
        use self::ColLetter::{A, B, C, D, E, F, G, H, J, K, L, M, N, P, Q, R, S, T, U, V, W, X, Y, Z};
        match set {
            1 => match *self {
                A => 0, B => 1, C => 2, D => 3, E => 4, F => 5, G => 6, H => 7,
                _ => panic!("Invalid e100k letter for set 1")
            },
            2 => match *self {
                J => 0, K => 1, L => 2, M => 3, N => 4, P => 5, Q => 6, R => 7,
                _ => panic!("Invalid e100k letter for set 2")
            },
            3 => match *self {
                S => 0, T => 1, U => 2, V => 3, W => 4, X => 5, Y => 6, Z => 7,
                _ => panic!("Invalid e100k letter for set 3")
            },
            _ => panic!("Invalid e100k set"),
        }
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