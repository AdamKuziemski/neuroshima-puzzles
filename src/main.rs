use clap::{Args, Parser, Subcommand};

use building_control::Building;
use guess_code::{Lock, LockError};

/// Pick a minigame
#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
pub struct Options {
    #[command(subcommand)]
    game: Minigame
}

#[derive(Subcommand, Debug)]
pub enum Minigame {
    Building(BuildingOptions),
    Lock(LockOptions),
}

/// Launches the building minigame
#[derive(Args, Debug)]
#[group(multiple = true)]
pub struct BuildingOptions {
    /// Defines the filename containing the building config
    #[arg(short, long)]
    file: String,
}

/// Launches the lock minigame
#[derive(Args, Debug)]
#[group(multiple = false)]
pub struct LockOptions {
    /// Defines the number of digits
    #[arg(short, long, default_value_t = 3)]
    digits: usize,

    /// Defines the code to guess
    #[arg(short, long)]
    code: Option<usize>,
}

fn main() -> Result<(), LockError> {
    let options = Options::parse();

    match options.game {
        Minigame::Lock(lock_options) => {
            match lock_options.code {
                Some(code) => {
                    let mut lock = Lock::from(code)?;
                    return lock.try_break();
                },
                None => ()
            }

            let mut lock = Lock::random(lock_options.digits)?;
            return lock.try_break();
        },
        Minigame::Building(building_options) => {
            match Building::from_file(building_options.file) {
                Ok(mut building) => building.manage(),
                Err(_) => println!("Error")
            }
            return Ok(())
        }
    }
}
