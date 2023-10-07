use clap::Args;

/// Launches the building minigame
#[derive(Args, Debug)]
#[group(multiple = true)]
pub struct BuildingConfig {
    /// Defines the filename containing the building config
    #[arg(short, long)]
    pub file: String,
}