use clap::Args;

/// Launches the lock minigame
#[derive(Args, Debug)]
#[group(multiple = false)]
pub struct LockConfig {
    /// Defines the number of digits
    #[arg(short, long, default_value_t = 3)]
    pub digits: usize,

    /// Defines the code to guess
    #[arg(short, long)]
    pub code: Option<usize>,
}