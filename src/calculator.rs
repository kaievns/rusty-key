use std::cell::Cell;
use crate::config::*;
use crate::keyboard::*;

type Coordinate = (usize, usize); // row, pos

#[derive(Debug)]
pub struct Calculator<'a> {
  keyboard: &'a Keyboard,
  previous_key: Cell<&'a Key>,
  bad_starters: Vec<Coordinate>
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Summary {
  pub effort: usize,
  pub distance: usize,
  pub overheads: usize
}

impl Calculator<'_> {
  pub fn from<'a>(keyboard: &'a Keyboard) -> Calculator {
    let space_key = keyboard.key_for(&' ').unwrap();
    let bad_starters = Self::calculate_bad_startes();

    Calculator { keyboard, previous_key: Cell::new(space_key), bad_starters }
  }

  fn calculate_bad_startes() -> Vec<Coordinate> {
    let querty = Keyboard::querty();
    let mut coordinates = vec![];

    for symbol in BAD_STARTERS_LIST.trim().split_whitespace() {
      let key = querty.key_for(&symbol.chars().next().unwrap()).unwrap();

      coordinates.push((key.row, key.pos));
    }

    coordinates
  } 

  pub fn run(self: &Self, text: &String) -> Summary {
    let mut effort: usize = 0;
    let mut distance: usize = 0;
    let mut overheads: usize = 0;

    for symbol in text.chars() {
      let key = self.keyboard.key_for(&symbol);

      match key {
        Some(key) => {
          distance += 1;
          effort += key.effort;

          let previous_key = self.previous_key.get();
          let same_key = previous_key == key;
          
          if !same_key && self.same_hand(previous_key, key) {
            let penalties = self.same_hand_penalties(previous_key, key);
            
            effort += penalties;
            overheads += penalties;
          }

          self.previous_key.set(key);
        },
        None => {},
      }
    }

    Summary { effort, distance, overheads }
  }

  fn same_hand_penalties(self: &Self, last_key: &Key, next_key: &Key) -> usize {
    let mut penalties = 0;
    
    if self.same_finger(last_key, next_key) {
      penalties += SAME_FINGER_PENALTY;
    }

    if self.bad_starter(last_key) {
      penalties += BAD_STARTER_PENALTY;
    }
    
    if !self.comfy_combo(last_key, next_key) {
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

  fn same_hand(self: &Self, previous_key: &Key, key: &Key) -> bool {
    previous_key.hand == key.hand
  }

  fn same_finger(self: &Self, last_key: &Key, next_key: &Key) -> bool {
    last_key.finger == next_key.finger
  }

  fn bad_starter(self: &Self, last_key: &Key) -> bool {
    let mut bad_starter = false;
    
    for coordinate in self.bad_starters.iter() {
      if self.same_place(coordinate, last_key) {
        bad_starter = true;
        break;
      }
    }

    bad_starter
  }

  fn comfy_combo(self: &Self, last_key: &Key, next_key: &Key) -> bool {
    // TODO implement me
    false
  }

  fn same_place(self: &Self, coordinate: &Coordinate, key: &Key) -> bool {
    let (row, pos) = coordinate;

    key.row == *row && key.pos == *pos
  }

  fn row_distance(self: &Self, last_key: &Key, next_key: &Key) -> usize {
    if last_key.row == 0 {
      0 // last key was space
    } else if last_key.row > next_key.row { 
      last_key.row - next_key.row 
    } else { 
      next_key.row - last_key.row 
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  fn run_text(text: &'static str) -> Summary {
    let keyboard = Keyboard::querty();
    let calculator = Calculator::from(&keyboard);
  
    calculator.run(&text.to_string())
  }

  #[test]
  fn calculates_basic() {
    assert_eq!(run_text("QUwiEOrp"), Summary {
      effort: 65,
      distance: 8,
      overheads: 0
    })
  }

  #[test]
  fn penalises_same_finger_usage() {
    let penalty = SAME_FINGER_PENALTY + ROW_JUMP_PENALTY;

    assert_eq!(run_text("fr"), Summary {
      effort: penalty + 6,
      distance: 2,
      overheads: penalty
    })
  }

  #[test]
  fn does_not_penalise_same_key_usage() {
    assert_eq!(run_text("ff"), Summary {
      effort: 0,
      distance: 2,
      overheads: 0
    })
  }

  #[test]
  fn penalises_row_jumps() {
    let penalty = ROW_JUMP_PENALTY;

    assert_eq!(run_text("vd"), Summary {
      effort: penalty + 6 + 0,
      distance: 2,
      overheads: penalty
    })
  }

  #[test]
  fn penalises_row_skips() {
    let penalty = ROW_SKIP_PENALTY;

    assert_eq!(run_text("vq"), Summary {
      effort: penalty + 6 + 6,
      distance: 2,
      overheads: penalty
    })
  }

  #[test]
  fn penalises_bad_starters() {
    assert_eq!(run_text("qw"), Summary {
      effort: BAD_STARTER_PENALTY + 6 + 2,
      distance: 2,
      overheads: BAD_STARTER_PENALTY
    });
  }

  #[test]
  fn doesnt_penalise_bad_starter_on_hand_switch() {
    assert_eq!(run_text("qi"), Summary {
      effort: 6 + 1,
      distance: 2,
      overheads: 0
    });
  }

  #[test]
  fn adds_extra_penalty_on_bad_starters_and_row_jump() {
    let penalty = BAD_STARTER_PENALTY + ROW_JUMP_PENALTY;

    assert_eq!(run_text("qs"), Summary {
      effort: penalty + 6 + 0,
      distance: 2,
      overheads: penalty
    });
  }

  #[test]
  fn adds_extra_penalty_on_bad_starters_and_skip_jump() {
    let penalty = BAD_STARTER_PENALTY + ROW_SKIP_PENALTY;

    assert_eq!(run_text("qv"), Summary {
      effort: penalty + 6 + 6,
      distance: 2,
      overheads: penalty
    });
  }

  #[test]
  fn adds_extra_penalty_on_bad_starters_and_same_finger() {
    let penalty = BAD_STARTER_PENALTY + ROW_SKIP_PENALTY + SAME_FINGER_PENALTY;

    assert_eq!(run_text("qz"), Summary {
      effort: penalty + 6 + 7,
      distance: 2,
      overheads: penalty
    });
  }
}
