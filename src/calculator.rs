use std::cell::Cell;
use std::collections::HashMap;
use crate::config::*;
use crate::keyboard::*;

type Coordinate = (usize, usize); // row, pos
type CoordinateMap = HashMap<Coordinate, Coordinate>;

#[derive(Debug)]
pub struct Calculator<'a> {
  keyboard: &'a Keyboard,
  previous_key: Cell<&'a Key>,
  bad_starters: Vec<Coordinate>,
  comfies: CoordinateMap
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Summary {
  pub effort: usize,
  pub distance: usize,
  pub overheads: usize,
  pub awkwardness: usize
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

fn calculate_comfies() -> CoordinateMap {
  let querty = Keyboard::querty();
  let mut map = CoordinateMap::new();

  for pair in COMFIES.trim().split_whitespace() {
    let mut chars = pair.chars();
    
    let first_letter = chars.next().unwrap();
    let second_letter = chars.next().unwrap();

    let first_key = querty.key_for(&first_letter).unwrap();
    let second_key = querty.key_for(&second_letter).unwrap();

    map.insert((first_key.row, first_key.pos), (second_key.row, second_key.pos));
  }

  map
}

fn same_hand(last_key: &Key, next_key: &Key) -> bool {
  last_key.hand == next_key.hand
}

fn same_finger(last_key: &Key, next_key: &Key) -> bool {
  last_key.finger == next_key.finger
}

fn same_place(coordinate: &Coordinate, key: &Key) -> bool {
  let (row, pos) = coordinate;

  key.row == *row && key.pos == *pos
}

fn row_distance(last_key: &Key, next_key: &Key) -> usize {
  if last_key.row == 0 {
    0 // last key was space
  } else if last_key.row > next_key.row { 
    last_key.row - next_key.row 
  } else { 
    next_key.row - last_key.row 
  }
}

impl Calculator<'_> {
  pub fn from<'a>(keyboard: &'a Keyboard) -> Calculator {
    let space_key = keyboard.key_for(&' ').unwrap();
    let bad_starters = calculate_bad_startes();
    let comfies = calculate_comfies();
    
    Calculator { keyboard, previous_key: Cell::new(space_key), bad_starters, comfies }
  }

  pub fn run(self: &Self, text: &String) -> Summary {
    let mut effort: usize = 0;
    let mut distance: usize = 0;
    let mut overheads: usize = 0;
    let mut awkwardness: usize = 0;

    for symbol in text.chars() {
      let key = self.keyboard.key_for(&symbol);

      match key {
        Some(key) => {
          distance += 1;
          effort += key.effort;

          let previous_key = self.previous_key.get();
          let same_key = previous_key == key;

          if !same_key && previous_key.row != 0 && same_hand(previous_key, key) {
            let same_hand_penalties = self.same_hand_penalties(previous_key, key);
            let awkwardness_penalty = self.awkward_penalty(previous_key, key);
            
            effort += same_hand_penalties + awkwardness_penalty;
            overheads += same_hand_penalties + awkwardness_penalty;
            awkwardness += awkwardness_penalty;
          }

          self.previous_key.set(key);
        },
        None => {},
      }
    }

    Summary { effort, distance, overheads, awkwardness }
  }

  fn same_hand_penalties(self: &Self, last_key: &Key, next_key: &Key) -> usize {
    let mut penalties = SAME_HAND_PENALTY;

    if same_finger(last_key, next_key) {
      penalties += SAME_FINGER_PENALTY;
    }
    
    if !self.comfy_combo(last_key, next_key) {
      match row_distance(last_key, next_key) {
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

  fn awkward_penalty(self: &Self, last_key: &Key, _next_key: &Key) -> usize {
    let mut penalties = 0;

    for coordinate in self.bad_starters.iter() {
      if same_place(coordinate, last_key) {
        penalties += BAD_STARTER_PENALTY;
        break;
      }
    }

    penalties
  }

  fn comfy_combo(self: &Self, last_key: &Key, next_key: &Key) -> bool {
    let mut comfy_combo = false;
    
    for (first_coord, second_coord) in self.comfies.iter() {
      if same_place(first_coord, last_key) && same_place(second_coord, next_key) {
        comfy_combo = true;
        break;
      }
    }
    
    comfy_combo
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
      overheads: 0,
      awkwardness: 0
    })
  }

  #[test]
  fn penalises_same_finger_usage() {
    let penalty = SAME_HAND_PENALTY + SAME_FINGER_PENALTY + ROW_JUMP_PENALTY;

    assert_eq!(run_text("fr"), Summary {
      effort: penalty + 6,
      distance: 2,
      overheads: penalty,
      awkwardness: 0
    })
  }

  #[test]
  fn does_not_penalise_same_key_usage() {
    assert_eq!(run_text("ff"), Summary {
      effort: 0,
      distance: 2,
      overheads: 0,
      awkwardness: 0
    })
  }

  #[test]
  fn penalises_row_jumps() {
    let penalty = SAME_HAND_PENALTY + ROW_JUMP_PENALTY;

    assert_eq!(run_text("vd"), Summary {
      effort: penalty + 6 + 0,
      distance: 2,
      overheads: penalty,
      awkwardness: 0
    })
  }

  #[test]
  fn penalises_row_skips() {
    let penalty = SAME_HAND_PENALTY + ROW_SKIP_PENALTY;

    assert_eq!(run_text("vq"), Summary {
      effort: penalty + 6 + 6,
      distance: 2,
      overheads: penalty,
      awkwardness: 0
    })
  }

  #[test]
  fn penalises_bad_starters() {
    let penalty = SAME_HAND_PENALTY + BAD_STARTER_PENALTY;

    assert_eq!(run_text("qw"), Summary {
      effort: penalty + 6 + 2,
      distance: 2,
      overheads: penalty,
      awkwardness: BAD_STARTER_PENALTY
    });
  }

  #[test]
  fn doesnt_penalise_bad_starter_on_hand_switch() {
    assert_eq!(run_text("qi"), Summary {
      effort: 6 + 1,
      distance: 2,
      overheads: 0,
      awkwardness: 0
    });
  }

  #[test]
  fn adds_extra_penalty_on_bad_starters_and_row_jump() {
    let penalty = SAME_HAND_PENALTY + BAD_STARTER_PENALTY + ROW_JUMP_PENALTY;

    assert_eq!(run_text("qs"), Summary {
      effort: penalty + 6 + 0,
      distance: 2,
      overheads: penalty,
      awkwardness: BAD_STARTER_PENALTY
    });
  }

  #[test]
  fn adds_extra_penalty_on_bad_starters_and_skip_jump() {
    let penalty = SAME_HAND_PENALTY + BAD_STARTER_PENALTY + ROW_SKIP_PENALTY;

    assert_eq!(run_text("qv"), Summary {
      effort: penalty + 6 + 6,
      distance: 2,
      overheads: penalty,
      awkwardness: BAD_STARTER_PENALTY
    });
  }

  #[test]
  fn adds_extra_penalty_on_bad_starters_and_same_finger() {
    let penalty = SAME_HAND_PENALTY + BAD_STARTER_PENALTY + ROW_SKIP_PENALTY + SAME_FINGER_PENALTY;

    assert_eq!(run_text("qz"), Summary {
      effort: penalty + 6 + 7,
      distance: 2,
      overheads: penalty,
      awkwardness: BAD_STARTER_PENALTY
    });
  }

  #[test]
  fn doesnt_penalise_comfies_for_row_jumps() {
    let penalty = SAME_HAND_PENALTY + SAME_HAND_PENALTY;

    assert_eq!(run_text("as;l"), Summary {
      effort: penalty + 2,
      distance: 4,
      overheads: penalty,
      awkwardness: 0
    });
  }
}
