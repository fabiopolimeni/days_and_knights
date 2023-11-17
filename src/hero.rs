#![allow(dead_code)]

use std::fmt;

pub const SPEED: f32 = 1.5;
pub const SPEED_MULTIPLIER: f32 = 2.5;
pub const MAX_SPEED: f32 = SPEED * SPEED_MULTIPLIER;
pub const MIN_MOVE_DISTANCE: f32 = 1.0;
pub const MAX_REMAINING_LOCOMOTION_TIME: f32 = 1. / 10.;

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
