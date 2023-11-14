use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Class {
    Barbarian,
    Knight,
    Mage,
    Rogue,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Class::Barbarian => write!(f, "Barbarian"),
            Class::Knight => write!(f, "Knight"),
            Class::Mage => write!(f, "Mage"),
            Class::Rogue => write!(f, "Rogue"),
        }
    }
}