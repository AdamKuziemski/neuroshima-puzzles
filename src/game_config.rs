use clap::Parser;

use crate::minigame::Minigame;

/// Lets the user pick a minigame
#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
pub struct GameConfig {
    #[command(subcommand)]
    pub game: Minigame
}