pub enum ManageMode {
    /// "Bare bones" view:
    /// | {Device Name} | {On/Off toggle} |
    Simple,
    /// Shows targeting mode:
    /// | {Device Name} | {Targeting} | {On/Off toggle} |
    Advanced,
    /// ???
    Professional,
}