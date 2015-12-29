use std::str;
use get_100k_set_for_zone;
use band::LatBand;
use col::ColLetter;
use row::RowLetter;

use Accuracy;
use Mgrs;

#[derive(Debug)]
pub struct MgrsParser<'a> {
    mgrs: &'a [u8],
    pos: usize,
    start: usize,
}

impl<'a> MgrsParser<'a> {
    pub fn new(mgrs: &'a [u8]) -> Self {
        MgrsParser {
            mgrs: mgrs,
            pos: 0,
            start: 0,
        }
    }

    pub fn parse(mut self) -> Mgrs {
        let mut mgrs = Mgrs { ..Default::default() };
        self.stop_at(numeric);
        self.zone_num(&mut mgrs);
        self.stop_at(zone_letter);
        self.zone_letter(&mut mgrs);
        self.stop_at(col_letter);
        self.col_letter(&mut mgrs);
        self.stop_at(row_letter);
        self.row_letter(&mut mgrs);
        self.stop_at(numeric);
        self.location(&mut mgrs);
        mgrs
    }

    fn stop_at<F>(&mut self, f: F) where F: Fn(u8) -> bool {
        self.start = self.pos;
        for b in &self.mgrs[self.start..] {
            if f(*b) { self.pos += 1; continue; }
            return;
        }
    }

    fn zone_num(&mut self, mgrs: &mut Mgrs) {
        self.start = self.pos;
        self.pos += 2;
        // returns true for non-numeric bytes
        let s_num = if !numeric(self.mgrs[self.pos]) {
            unsafe { str::from_utf8_unchecked(&self.mgrs[self.start..self.pos]) }
        } else {
            self.pos -= 1;
            unsafe { str::from_utf8_unchecked(&self.mgrs[self.start..self.pos]) }
        };
        mgrs.gzd.zone = s_num.parse().expect("Failed to parse bytes to number in MGRS string");
    }

    fn zone_letter(&mut self, mgrs: &mut Mgrs) {
        self.pos += 1;
        let c = self.mgrs[self.pos] as char;
        mgrs.gzd.band = LatBand::from(c);
    }

    fn col_letter(&mut self, mgrs: &mut Mgrs) {
        self.pos += 1;
        let c = self.mgrs[self.pos] as char;
        mgrs.gsid_100k.col = ColLetter::from(c);
    }

    fn row_letter(&mut self, mgrs: &mut Mgrs) {
        self.pos += 1;
        let c = self.mgrs[self.pos] as char;
        mgrs.gsid_100k.row = RowLetter::from(c);
    }

    fn location(&mut self, mgrs: &mut Mgrs) {
        self.start = self.pos;
        let loc = &self.mgrs[self.start..self.mgrs.len()];

        let (e, n) = if !contains_whitespace(loc) {
            assert!(loc.len() % 2 == 0, "Odd number of digits for MGRS grid");
            let e = &loc[..loc.len()/2];
            let n = &loc[loc.len()/2..];
            (e, n)
        } else {
            self.stop_at(whitespace);
            self.pos += 1;
            let e = &self.mgrs[self.start..self.pos];
            self.start = self.pos;
            self.stop_at(numeric);
            let n = &self.mgrs[self.start..];
            (e, n)
        };

        mgrs.accuracy = Accuracy::from_num_digits(loc.len()).expect("Failed to retrieve accuracy");

        let set = get_100k_set_for_zone(mgrs.gzd.zone as usize);

        let e_100k = mgrs.gsid_100k.col.get_easting_with_set(set as u8);
        let mut n_100k = mgrs.gsid_100k.row.get_northing_with_set(set as u8);

        // We have a bug where the northing may be 2000000 too low.
        // How do we know when to roll over?

        let min_n = mgrs.gzd.band.get_min_northing().expect("faild to get min northing");
        while n_100k < min_n {
            n_100k += 2000000.0;
        }

        let base: usize = 10;
        let accuracy_bonus: f64 = 100000.0 / base.pow(mgrs.accuracy.as_num_digits() as u32) as f64;
        let e_str = unsafe { str::from_utf8_unchecked(e) };
        let n_str = unsafe { str::from_utf8_unchecked(n) };
        let ef = e_str.parse::<f64>().expect("failed to parse easting in MGRS string") * accuracy_bonus;
        let nf = n_str.parse::<f64>().expect("failed to parse northing in MGRS string") * accuracy_bonus;

        mgrs.easting = ef + e_100k;
        mgrs.northing = nf + n_100k;
    }
}

#[inline]
fn numeric(b: u8) -> bool {
    // 48=0, 57=9
    b < 48 || b > 57
}

#[inline]
fn zone_letter(b: u8) -> bool {
    // C-X, except I and O
    b < 67 || b > 120 || (b > 88 && b < 99) || exempt_letters(b)
}

#[inline]
fn col_letter(b: u8) -> bool {
    // A-Z, except I and O
    b < 65 || b > 122 || (b > 90 && b < 97) || exempt_letters(b)
}

#[inline]
fn row_letter(b: u8) -> bool {
    // A-V, except I and O
    b < 65 || b > 118 || (b > 86 && b < 97) || exempt_letters(b)
}

#[inline]
fn exempt_letters(b: u8) -> bool {
    b == b'I' || b == b'O' || b == b'o' || b == b'i'
}

#[inline]
fn whitespace(b: u8) -> bool {
    b < 33
}
#[inline]
fn contains_whitespace(bytes: &[u8]) -> bool {
    for b in bytes {
        if *b < 33 { return true; }
    }
    false
}
