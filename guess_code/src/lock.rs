use std::cmp::Ordering;
use std::io;
use rand::Rng;

use crate::lock_length::LockLength;
use crate::lock_error::LockError;

pub struct Lock {
    pub code: usize,
    pub digits: LockLength,
    pub max_guesses: usize,
    pub shorts: usize,
    pub tries: usize,
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
        self.introduce_game();

        while self.tries < self.max_guesses {
            self.show_try_count();

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


    fn introduce_game(&self) {
        self.clear_screen();
        println!("Złam {}-cyfrowy szyfr! Masz {} prób.", self.digits.as_usize(), self.max_guesses);
    }

    fn clear_screen(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    fn show_try_count(&self) {
        println!("--------");
        println!("Próba {}:", self.tries + 1);
    }
}