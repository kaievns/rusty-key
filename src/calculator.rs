use crate::config::*;
use crate::parser::Position;
use crate::geometry::{Key};
use crate::keyboard::*;

use hashbrown::{HashSet};
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct Calculator<'a> {
  keyboard: &'a Keyboard<'a>,
  bad_starters: &'a HashSet<Position>,
  rolling_pairs_map: &'a HashSet<(Position, Position)>
}

#[derive(Debug,PartialEq)]
pub struct Result {
  pub effort: f64,
  pub overheads: f64,
  pub awkwardness: f64,
  pub rollingness: f64
}

pub fn process(keyboard: &Keyboard) -> Result {
  let calculator = Calculator::from(keyboard);
  calculator.run(&CONFIG.data)
}

struct MappingsSet {
  bad_starters: HashSet<Position>,
  rolling_pairs: HashSet<(Position, Position)>
}

static MAPPINGS_CACHE: Lazy<MappingsSet> = Lazy::new(|| MappingsSet {
  bad_starters: CONFIG.geometry.bad_starting_positions(),
  rolling_pairs: CONFIG.geometry.rolling_position_pairs()
});

impl Calculator<'_> {
  pub fn from<'a>(keyboard: &'a Keyboard) -> Calculator<'a> {
    let bad_starters = &MAPPINGS_CACHE.bad_starters;
    let rolling_pairs_map = &MAPPINGS_CACHE.rolling_pairs;
    
    Calculator { keyboard, bad_starters, rolling_pairs_map }
  }

  pub fn run(self: &Self, text: &String) -> Result {
    let mut effort: usize = 0;
    let mut overheads: usize = 0;
    let mut awkwardness: usize = 0;
    let mut rollingness: usize = 0;
    
    let space_key = self.keyboard.key_for(&' ').unwrap();

    let mut previous_key = space_key;
    let mut rolling = false;
    let rolling_discount = 100 - CONFIG.penalties.rolling_discount;

    for symbol in text.chars() {
      if let Some(key) = self.keyboard.key_for(&symbol) {
        effort += key.effort;

        if 
          previous_key.hand == key.hand && // same hand
          key != previous_key &&           // different key
          key != space_key &&              // not space
          previous_key != space_key        // not from space
        {
          let mut same_hand_penalties = self.same_hand_penalties(previous_key, key);
          let mut awkwardness_penalty = self.awkward_penalty(previous_key, key);

          // only allowing two rolling symbols in a row to awoid counting the change of direction as a roll
          rolling = if rolling { false } else { self.is_rolling_combo(previous_key, key) };
        
          if rolling {
            rollingness += 1;

            same_hand_penalties = (same_hand_penalties * rolling_discount) / 100;
            awkwardness_penalty = (awkwardness_penalty  * rolling_discount) / 100;
          }


          effort += same_hand_penalties + awkwardness_penalty;
          overheads += same_hand_penalties + awkwardness_penalty;
          awkwardness += awkwardness_penalty;
        }

        previous_key = key;
      }
    }

    // turning everything into coefficents against the text length
    Result { 
      effort: (effort as f64) / (text.len() as f64), 
      overheads: (overheads as f64) / (text.len() as f64), 
      awkwardness: (awkwardness as f64) / (text.len() as f64), 
      rollingness: (rollingness as f64) / (text.len() as f64) 
    }
  }

  fn same_hand_penalties(self: &Self, last_key: &Key, next_key: &Key) -> usize {
    let mut penalties = CONFIG.penalties.same_hand;

    if last_key.finger == next_key.finger {
      penalties += CONFIG.penalties.same_finger;
    }
    
    match self.row_distance(last_key, next_key) {
      2 => penalties + CONFIG.penalties.row_skip,
      1 => penalties + CONFIG.penalties.row_jump,
      _ => penalties
    }
  }

  fn row_distance(self: &Self, last_key: &Key, next_key: &Key) -> usize {
    let last_row = last_key.position.0;
    let next_row = next_key.position.0;
  
    if last_row > next_row { 
      last_row - next_row 
    } else { 
      next_row - last_row
    }
  }  

  fn awkward_penalty(self: &Self, last_key: &Key, _next_key: &Key) -> usize {
    if self.bad_starters.contains(&last_key.position) {
      CONFIG.penalties.bad_starter
    } else {
      0
    }
  }

  fn is_rolling_combo(self: &Self, last_key: &Key, next_key: &Key) -> bool {
    let pair = (last_key.position, next_key.position);

    self.rolling_pairs_map.contains(&pair)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::layout::QWERTY;
  use crate::geometry::US_PC_KEYBOARD;

  fn run_text(text: &'static str) -> Result {
    let layout = QWERTY.clone();
    let keyboard = Keyboard::from(&layout, &US_PC_KEYBOARD);
    let calculator = Calculator::from(&keyboard);
  
    calculator.run(&text.to_string())
  }

  #[test]
  fn calculates_basic() {
    assert_eq!(run_text("QUwiEOrp"), Result {
      effort: (67 as f64) / 8.0,
      overheads: 0.0,
      awkwardness: 0.0,
      rollingness: 0.0
    })
  }

  #[test]
  fn penalises_same_finger_usage() {
    let penalty = CONFIG.penalties.same_hand + CONFIG.penalties.same_finger + CONFIG.penalties.row_jump;

    assert_eq!(run_text("fr"), Result {
      effort: ((penalty + 6) as f64) / 2.0,
      overheads: (penalty as f64) / 2.0,
      awkwardness: 0.0,
      rollingness: 0.0
    })
  }

  #[test]
  fn does_not_penalise_same_key_usage() {
    assert_eq!(run_text("ff"), Result {
      effort: 0.0,
      overheads: 0.0,
      awkwardness: 0.0,
      rollingness: 0.0
    })
  }

  #[test]
  fn penalises_row_jumps() {
    let penalty = CONFIG.penalties.same_hand + CONFIG.penalties.row_jump;

    assert_eq!(run_text("at"), Result {
      effort: ((penalty + 1 + 11) as f64) / 2.0,
      overheads: (penalty as f64) / 2.0,
      awkwardness: 0.0,
      rollingness: 0.0
    })
  }

  #[test]
  fn penalises_row_skips() {
    let penalty = CONFIG.penalties.same_hand + CONFIG.penalties.row_skip;

    assert_eq!(run_text("vq"), Result {
      effort: ((penalty + 6 + 6) as f64) / 2.0,
      overheads: (penalty as f64) / 2.0,
      awkwardness: 0.0,
      rollingness: 0.0
    })
  }

  #[test]
  fn penalises_bad_starters() {
    let penalty = CONFIG.penalties.same_hand + CONFIG.penalties.bad_starter;

    assert_eq!(run_text("qw"), Result {
      effort: ((penalty + 6 + 2) as f64) / 2.0,
      overheads: (penalty as f64) / 2.0,
      awkwardness: (CONFIG.penalties.bad_starter as f64) / 2.0,
      rollingness: 0.0
    });
  }

  #[test]
  fn doesnt_penalise_bad_starter_on_hand_switch() {
    assert_eq!(run_text("qi"), Result {
      effort: ((6 + 1) as f64) / 2.0,
      overheads: 0.0,
      awkwardness: 0.0,
      rollingness: 0.0
    });
  }

  #[test]
  fn adds_extra_penalty_on_bad_starters_and_row_jump() {
    let penalty = CONFIG.penalties.same_hand + CONFIG.penalties.bad_starter + CONFIG.penalties.row_jump;

    assert_eq!(run_text("qs"), Result {
      effort: ((penalty + 6 + 0) as f64) / 2.0,
      overheads: (penalty as f64) / 2.0,
      awkwardness: (CONFIG.penalties.bad_starter as f64) / 2.0,
      rollingness: 0.0
    });
  }

  #[test]
  fn adds_extra_penalty_on_bad_starters_and_skip_jump() {
    let penalty = CONFIG.penalties.same_hand + CONFIG.penalties.bad_starter + CONFIG.penalties.row_skip;

    assert_eq!(run_text("qv"), Result {
      effort: ((penalty + 6 + 6) as f64) / 2.0,
      overheads: (penalty as f64) / 2.0,
      awkwardness: (CONFIG.penalties.bad_starter as f64) / 2.0,
      rollingness: 0.0
    });
  }

  #[test]
  fn adds_extra_penalty_on_bad_starters_and_same_finger() {
    let penalty = CONFIG.penalties.same_hand + CONFIG.penalties.bad_starter + CONFIG.penalties.row_skip + CONFIG.penalties.same_finger;

    assert_eq!(run_text("qz"), Result {
      effort: ((penalty + 6 + 7) as f64) / 2.0,
      overheads: (penalty as f64) / 2.0,
      awkwardness: (CONFIG.penalties.bad_starter as f64) / 2.0,
      rollingness: 0.0
    });
  }

  #[test]
  fn doesnt_penalise_rolling_pairs_for_row_jumps() {
    // let penalty = CONFIG.penalties.same_hand + CONFIG.penalties.same_hand;

    assert_eq!(run_text("wfli"), Result {
      effort: 11.25, 
      overheads: 10.5, 
      awkwardness: 0.0, 
      rollingness: 0.25
    });
  } 
}
