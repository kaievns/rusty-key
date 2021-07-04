use crate::config::*;
use crate::parser::Position;
use crate::geometry::{Key};
use crate::keyboard::*;
use crate::summary::*;

use hashbrown::HashSet;

#[derive(Debug)]
pub struct Calculator<'a> {
  keyboard: &'a Keyboard<'a>,
  bad_starters: HashSet<Position>,
  rolling_pairs_map: HashSet<(Position, Position)>
}

impl Calculator<'_> {
  pub fn from<'a>(keyboard: &'a Keyboard) -> Calculator<'a> {
    let bad_starters = keyboard.geometry.bad_starting_positions();
    let rolling_pairs_map = keyboard.geometry.rolling_position_pairs();
    
    Calculator { keyboard, bad_starters, rolling_pairs_map }
  }

  pub fn run(self: &Self, text: &String) -> Summary {
    let mut effort: usize = 0;
    let mut overheads: usize = 0;
    let mut awkwardness: usize = 0;
    let mut rollingness: usize = 0;
    
    let space_key = self.keyboard.key_for(&' ').unwrap();

    let mut previous_key = space_key;

    for symbol in text.chars() {
      let key = self.keyboard.key_for(&symbol);

      match key {
        Some(key) => {
          effort += key.effort;

          if 
            previous_key.hand == key.hand && // same hand
            key != previous_key &&           // different key
            key != space_key &&              // not space
            previous_key != space_key        // not from space
          {
            let rolling = self.is_rolling_combo(previous_key, key);
          
            if rolling {
              rollingness += 1;
            }

            let same_hand_penalties = self.same_hand_penalties(previous_key, key, rolling);
            let awkwardness_penalty = self.awkward_penalty(previous_key, key, rolling);

            effort += same_hand_penalties + awkwardness_penalty;
            overheads += same_hand_penalties + awkwardness_penalty;
            awkwardness += awkwardness_penalty;
          }

          previous_key = key;
        },
        None => {},
      }
    }

    Summary { effort, overheads, awkwardness, rollingness }
  }

  fn same_hand_penalties(self: &Self, last_key: &Key, next_key: &Key, rolling: bool) -> usize {
    let mut penalties = SAME_HAND_PENALTY;

    if last_key.finger == next_key.finger {
      penalties += SAME_FINGER_PENALTY;
    }
    
    if !rolling {
      match self.row_distance(last_key, next_key) {
        2 => {
          penalties += ROW_SKIP_PENALTY;
        },
        1 => {
          penalties += ROW_JUMP_PENALTY;
        },
        _ => {}
      }
    }

    penalties
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

  fn awkward_penalty(self: &Self, last_key: &Key, _next_key: &Key, rolling: bool) -> usize {
    if !rolling && self.bad_starters.contains(&last_key.position) {
      BAD_STARTER_PENALTY
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
  use crate::layout::{ Layout, QWERTY };
  use crate::geometry::US_PC_KEYBOARD;

  fn run_text(text: &'static str) -> Summary {
    let layout = Layout { template: QWERTY.to_string() };
    let keyboard = Keyboard::from(&layout, &US_PC_KEYBOARD);
    let calculator = Calculator::from(&keyboard);
  
    calculator.run(&text.to_string())
  }

  #[test]
  fn calculates_basic() {
    assert_eq!(run_text("QUwiEOrp"), Summary {
      effort: 67,
      overheads: 0,
      awkwardness: 0,
      rollingness: 0
    })
  }

  #[test]
  fn penalises_same_finger_usage() {
    let penalty = SAME_HAND_PENALTY + SAME_FINGER_PENALTY + ROW_JUMP_PENALTY;

    assert_eq!(run_text("fr"), Summary {
      effort: penalty + 6,
      overheads: penalty,
      awkwardness: 0,
      rollingness: 0
    })
  }

  #[test]
  fn does_not_penalise_same_key_usage() {
    assert_eq!(run_text("ff"), Summary {
      effort: 0,
      overheads: 0,
      awkwardness: 0,
      rollingness: 0
    })
  }

  #[test]
  fn penalises_row_jumps() {
    let penalty = SAME_HAND_PENALTY + ROW_JUMP_PENALTY;

    assert_eq!(run_text("at"), Summary {
      effort: penalty + 1 + 11,
      overheads: penalty,
      awkwardness: 0,
      rollingness: 0
    })
  }

  #[test]
  fn penalises_row_skips() {
    let penalty = SAME_HAND_PENALTY + ROW_SKIP_PENALTY;

    assert_eq!(run_text("vq"), Summary {
      effort: penalty + 6 + 6,
      overheads: penalty,
      awkwardness: 0,
      rollingness: 0
    })
  }

  #[test]
  fn penalises_bad_starters() {
    let penalty = SAME_HAND_PENALTY + BAD_STARTER_PENALTY;

    assert_eq!(run_text("qw"), Summary {
      effort: penalty + 6 + 2,
      overheads: penalty,
      awkwardness: BAD_STARTER_PENALTY,
      rollingness: 0
    });
  }

  #[test]
  fn doesnt_penalise_bad_starter_on_hand_switch() {
    assert_eq!(run_text("qi"), Summary {
      effort: 6 + 1,
      overheads: 0,
      awkwardness: 0,
      rollingness: 0
    });
  }

  #[test]
  fn adds_extra_penalty_on_bad_starters_and_row_jump() {
    let penalty = SAME_HAND_PENALTY + BAD_STARTER_PENALTY + ROW_JUMP_PENALTY;

    assert_eq!(run_text("qs"), Summary {
      effort: penalty + 6 + 0,
      overheads: penalty,
      awkwardness: BAD_STARTER_PENALTY,
      rollingness: 0
    });
  }

  #[test]
  fn adds_extra_penalty_on_bad_starters_and_skip_jump() {
    let penalty = SAME_HAND_PENALTY + BAD_STARTER_PENALTY + ROW_SKIP_PENALTY;

    assert_eq!(run_text("qv"), Summary {
      effort: penalty + 6 + 6,
      overheads: penalty,
      awkwardness: BAD_STARTER_PENALTY,
      rollingness: 0
    });
  }

  #[test]
  fn adds_extra_penalty_on_bad_starters_and_same_finger() {
    let penalty = SAME_HAND_PENALTY + BAD_STARTER_PENALTY + ROW_SKIP_PENALTY + SAME_FINGER_PENALTY;

    assert_eq!(run_text("qz"), Summary {
      effort: penalty + 6 + 7,
      overheads: penalty,
      awkwardness: BAD_STARTER_PENALTY,
      rollingness: 0
    });
  }

  #[test]
  fn doesnt_penalise_rolling_pairs_for_row_jumps() {
    let penalty = SAME_HAND_PENALTY + SAME_HAND_PENALTY;

    assert_eq!(run_text("wfli"), Summary {
      effort: penalty + 3,
      overheads: penalty,
      awkwardness: 0,
      rollingness: 2
    });
  } 
}
