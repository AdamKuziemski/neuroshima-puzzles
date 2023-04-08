use std::cmp::Ordering;
use std::io;
use rand::Rng;

pub struct Lock {
    code: usize,
    digits: LockLength,
    max_guesses: usize,
    shorts: usize,
    tries: usize,
}

pub enum LockError {
    Invalid,
    TooHard,
    TooShort,
    Shorted,
    FailedToOpen
}

enum LockLength {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven
}

impl Lock {
    pub fn random(digits: usize) -> Result<Self, LockError> {
        let digits = LockLength::new(digits)?;
        let max_range = digits.as_range();
        let max_guesses: usize = digits.guess_range();
        let code: usize = rand::thread_rng().gen_range(0..max_range);

        Ok(Self {
            code,
            digits,
            max_guesses,
            shorts: 0,
            tries: 0,
        })
    }

    pub fn from(code: usize) -> Result<Self, LockError> {
        let digits = LockLength::from(code)?;
        let max_guesses: usize = digits.guess_range();

        Ok(Self {
            code,
            digits,
            max_guesses,
            shorts: 0,
            tries: 0,
        })
    }

    pub fn try_break(&mut self) -> Result<(), LockError> {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear screen
        println!("Złam {}-cyfrowy szyfr! Masz {} prób.", self.digits.as_usize(), self.max_guesses);

        while self.tries < self.max_guesses {
            println!("--------");
            println!("Próba {}:", self.tries + 1);

            let previous_shorts = self.shorts;
            let guess = self.get_input()?;

            if self.shorts == previous_shorts {
                match guess.cmp(&self.code) {
                    Ordering::Less => println!("Liczba zbyt niska!"),
                    Ordering::Greater => println!("Liczba zbyt wysoka!"),
                    Ordering::Equal => {
                        println!("Zamek się otwiera. Sukces!");
                        return Ok(());
                    },
                }
            }

            self.tries += 1;
        }

        Err(LockError::FailedToOpen)
    }

    fn get_input(&mut self) -> Result<usize, LockError> {
        let mut guess = String::new();

        let line = io::stdin()
            .read_line(&mut guess);

        match line {
            Ok(_) => self.parse(&guess),
            _ => Err(LockError::Shorted)
        }
    }

    fn parse(&mut self, guess: &str) -> Result<usize, LockError> {
        let guess = guess.trim();

        match guess.trim().parse() {
            Ok(value) if !self.is_shorted(&guess) => Ok(value),
            _ => Err(LockError::Shorted)
        }
    }

    fn is_shorted(&mut self, guess: &str) -> bool {
        if guess.len() < self.digits.as_usize() {
            println!("Kod za krótki! [~Bzzz~]");
            self.shorts += 1;
        }

        if guess.len() > self.digits.as_usize() {
            println!("Kod za długi! [~Bzzz~]");
            self.shorts += 1;
        }

        self.shorts == 3
    }
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

impl std::fmt::Debug for LockError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            LockError::FailedToOpen =>
                write!(f, "Nie udało się otworzyć zamka. Porażka!"),
            LockError::Shorted =>
                write!(f, "Nastąpiło zwarcie. Porażka!"),
            LockError::TooHard => 
                write!(f, "Kod zbyt trudny."),
            LockError::TooShort => 
                write!(f, "Kod powinien być co najmniej dwucyfrowy."),
            LockError::Invalid => 
                write!(f, "Nieprawidłowe parametry kodu")
        }
    }
}
