use crate::lock_error::LockError;

pub enum LockLength {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven
}

impl LockLength {
    pub fn new(digits: usize) -> Result<Self, LockError> {
        match digits {
            0 | 1 => Err(LockError::TooShort),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            6 => Ok(Self::Six),
            7 => Ok(Self::Seven),
            _ => Err(LockError::TooHard)
        }
    }

    pub fn from(number: usize) -> Result<Self, LockError> {
        let mut digits: usize = 0;

        while 10_usize.pow(digits as u32) <= number {
            digits = digits + 1;
        }

        Self::new(digits)
    }

    pub fn as_range(&self) -> usize {
        let mut range = 1;
        let mut i: usize = 0;

        while i < self.as_usize() {
            range *= 10;
            i += 1;
        }

        range
    }

    pub fn guess_range(&self) -> usize {
        match self {
            Self::Two => 7,
            Self::Three => 10,
            Self::Four => 14,
            Self::Five => 17,
            Self::Six => 20,
            Self::Seven => 24
        }
    }

    pub fn as_usize(&self) -> usize {
        match *self {
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7
        }
    }
}