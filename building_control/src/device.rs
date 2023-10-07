use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Device {
    pub name: String,
    pub is_on: bool,
    pub targets_humans: bool,
    pub targets_mutants: bool,
    pub targets_machines: bool,
}