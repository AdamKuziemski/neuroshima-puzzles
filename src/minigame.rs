use clap::Subcommand;

use crate::building_config::BuildingConfig;
use crate::lock_config::LockConfig;

#[derive(Subcommand, Debug)]
pub enum Minigame {
    Building(BuildingConfig),
    Lock(LockConfig),
}