#![allow(dead_code)]

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

pub const SPEED: f32 = 0.15;
pub const SPEED_MULTIPLIER: f32 = 2.0;
pub const MAX_SPEED: f32 = SPEED * SPEED_MULTIPLIER;
pub const MIN_MOVE_DISTANCE: f32 = 1.0;