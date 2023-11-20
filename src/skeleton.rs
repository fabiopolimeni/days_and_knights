#![allow(dead_code)]

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkeletonClass {
    Minon,
    Warrior,
    Shaman,
    Archer,
}

impl fmt::Display for SkeletonClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SkeletonClass::Minon => write!(f, "Minon"),
            SkeletonClass::Warrior => write!(f, "Warrior"),
            SkeletonClass::Shaman => write!(f, "Shaman"),
            SkeletonClass::Archer => write!(f, "Archer"),
        }
    }
}
