pub enum LockError {
    Invalid,
    TooHard,
    TooShort,
    Shorted,
    FailedToOpen
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