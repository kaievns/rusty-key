use crate::geometry::*;

pub const DEFAULT_GEOMETRY: Geometry = US_PC_KEYBOARD;

pub const SAME_HAND_PENALTY: usize = 5;
pub const SAME_FINGER_PENALTY: usize = 100;
pub const BAD_STARTER_PENALTY: usize = 80;
pub const ROW_SKIP_PENALTY: usize = 50;
pub const ROW_JUMP_PENALTY: usize = 30;

pub const POPULATION_SIZE: usize = 10;
pub const MUTATE_SYMBOLS: bool = false;

pub const PRESERVED_SYMBOLS: &'static str = "";