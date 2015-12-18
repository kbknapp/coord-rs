/// Accuracy in meters
#[derive(Debug, Copy, Clone)]
pub enum Accuracy {
    One,            // 10 digit
    Ten,            // 8 digit
    OneHundred,     // 6 digit
    OneThousand,    // 4 digit
    TenThousand,    // 2 digit
}

impl Accuracy {
    pub fn from_numeric(a: usize) -> Option<Self> {
        // 1 - One
        // 2 - Ten
        // 3 - OneHundred, etc.
        match a {
            1 => Some(Accuracy::One),
            2 => Some(Accuracy::Ten),
            3 => Some(Accuracy::OneHundred),
            4 => Some(Accuracy::OneThousand),
            5 => Some(Accuracy::TenThousand),
            _ => None
        }
    }

    pub fn from_distance(a: usize) -> Option<Self> {
        // 1 - One
        // 10 - Ten
        // 100 - OneHundred, etc.
        match a {
            1 => Some(Accuracy::One),
            10 => Some(Accuracy::Ten),
            100 => Some(Accuracy::OneHundred),
            1000 => Some(Accuracy::OneThousand),
            10000 => Some(Accuracy::TenThousand),
            _ => None
        }
    }

    pub fn from_num_digits(a: usize) -> Option<Self> {
        // 10 - 1
        // 8 - Ten
        // 6 - OneHundred, etc.
        match a {
            10 => Some(Accuracy::One),
            8 => Some(Accuracy::Ten),
            6 => Some(Accuracy::OneHundred),
            4 => Some(Accuracy::OneThousand),
            2 => Some(Accuracy::TenThousand),
            _ => None
        }
    }

    pub fn as_numeric(&self) -> usize {
        // 1 - One
        // 2 - Ten
        // 3 - OneHundred, etc.
        match *self {
            Accuracy::One => 1,
            Accuracy::Ten => 2,
            Accuracy::OneHundred => 3,
            Accuracy::OneThousand => 4,
            Accuracy::TenThousand => 5,
        }
    }

    pub fn as_distance(&self) -> usize {
        // 1 - One
        // 10 - Ten
        // 100 - OneHundred, etc.
        match *self {
            Accuracy::One => 1,
            Accuracy::Ten => 10,
            Accuracy::OneHundred => 100,
            Accuracy::OneThousand => 1000,
            Accuracy::TenThousand => 10000,
        }
    }

    pub fn as_num_digits(&self) -> usize {
        // 10 - 1
        // 8 - Ten
        // 6 - OneHundred, etc.
        match *self {
            Accuracy::One => 10,
            Accuracy::Ten => 8,
            Accuracy::OneHundred => 6,
            Accuracy::OneThousand => 4,
            Accuracy::TenThousand => 2,
        }
    }
}

impl Default for Accuracy {
    fn default() -> Self {
        Accuracy::One
    }
}
