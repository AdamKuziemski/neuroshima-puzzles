use clap::{Parser};

use building_control::Building;
use guess_code::{Lock, LockError};

mod game_config;
use game_config::GameConfig;

mod minigame;
use minigame::Minigame;

mod building_config;
mod lock_config;

fn main() -> Result<(), LockError> {
    let options = GameConfig::parse();

    match options.game {
        Minigame::Lock(lock_config) => {
            match lock_config.code {
                Some(code) => {
                    let mut lock = Lock::from(code)?;
                    return lock.try_break();
                },
                None => ()
            }

            let mut lock = Lock::random(lock_config.digits)?;
            return lock.try_break();
        },
        Minigame::Building(building_config) => {
            match Building::from_file(building_config.file) {
                Ok(building) => building.manage(),
                Err(_) => println!("Error")
            }
            return Ok(())
        }
    }
}
