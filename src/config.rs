use crate::source;
use crate::geometry::*;
use crate::preservative::*;
use crate::frequency::*;

pub const SAME_HAND_PENALTY: usize = 5;
pub const SAME_FINGER_PENALTY: usize = 100;
pub const BAD_STARTER_PENALTY: usize = 80;
pub const ROW_SKIP_PENALTY: usize = 50;
pub const ROW_JUMP_PENALTY: usize = 30;

pub const POPULATION_SIZE: usize = 30;

lazy_static! {
  pub static ref CONFIG: Config = Config::defaults();
}

pub struct Config {
  pub geometry: Geometry,
  pub preserve: Preservative,
  pub data: String,
  pub symbol_freaquencies: SymbolFrequencies
}

impl Config {
  pub fn defaults() -> Config {
    let geometry = US_PC_KEYBOARD;
    let preserve = Preservative::default();
    let data = source::load_english_text();
    let symbol_freaquencies = ENGLISH_TEXT_FREQUENCIES;

    Config { geometry, preserve, data, symbol_freaquencies }
  }
}