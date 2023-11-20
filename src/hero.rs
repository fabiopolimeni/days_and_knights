#![allow(dead_code)]

use std::fmt;

pub const SPEED: f32 = 0.1;
pub const SPEED_MULTIPLIER: f32 = 2.;
pub const MAX_SPEED: f32 = SPEED * SPEED_MULTIPLIER;
pub const MIN_MOVE_DISTANCE: f32 = 0.2;
pub const MAX_REMAINING_LOCOMOTION_TIME: f32 = 1. / 10.;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HeroClass {
    Barbarian,
    Knight,
    Mage,
    Rogue,
}

impl fmt::Display for HeroClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HeroClass::Barbarian => write!(f, "Barbarian"),
            HeroClass::Knight => write!(f, "Knight"),
            HeroClass::Mage => write!(f, "Mage"),
            HeroClass::Rogue => write!(f, "Rogue"),
        }
    }
}
