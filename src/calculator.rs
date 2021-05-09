use std::cell::Cell;
use crate::config::*;
use crate::keyboard::*;
use crate::summary::*;

type CoordinatePair = (Coordinates, Coordinates);
type CoordinatePairs = Vec<CoordinatePair>;

#[derive(Debug)]
pub struct Calculator<'a> {
  keyboard: &'a Keyboard,
  previous_key: Cell<&'a Key>,
  bad_starters: Vec<Coordinates>,
  comfies_map: CoordinatePairs
}

fn calculate_bad_startes() -> Vec<Coordinates> {
  let querty = Keyboard::querty();
  let mut coordinates = vec![];

  for symbol in BAD_STARTERS_LIST.trim().split_whitespace() {
    let key = querty.key_for(&symbol.chars().next().unwrap()).unwrap();

    coordinates.push(key.coords);
  }

  coordinates
}

fn calculate_comfies() -> CoordinatePairs {
  let querty = Keyboard::querty();
  let mut pairs = vec![];

  for pair in COMFIES.trim().split_whitespace() {
    let mut chars = pair.chars();

    let first_letter = chars.next().unwrap();
    let second_letter = chars.next().unwrap();

    let first_key = querty.key_for(&first_letter).unwrap();
    let second_key = querty.key_for(&second_letter).unwrap();

    pairs.push((first_key.coords, second_key.coords));
  }

  pairs
}

fn row_distance(last_key: &Key, next_key: &Key) -> usize {
  let last_row = last_key.coords.0;
  let next_row = next_key.coords.0;

  if last_row == 0 {
    0 // last key was space
  } else if last_row > next_row { 
    last_row - next_row 
  } else { 
    next_row - last_row 
  }
}

fn record_usage_map(usage: &mut UsageMap, key: &Key) {
  let count = usage.entry(key.coords).or_insert(0);

  *count += 1;
}

impl Calculator<'_> {
  pub fn from<'a>(keyboard: &'a Keyboard) -> Calculator {
    let space_key = keyboard.key_for(&' ').unwrap();
    let bad_starters = calculate_bad_startes();
    let comfies_map = calculate_comfies();
    
    Calculator { keyboard, previous_key: Cell::new(space_key), bad_starters, comfies_map }
  }

  pub fn run(self: &Self, text: &String) -> Summary {
    let really_big_limit = 99999999999999;
    self.run_to_limit(text, really_big_limit)
  }

  pub fn run_to_limit(self: &Self, text: &String, effort_limit: usize) -> Summary {
    let mut effort: usize = 0;
    let mut distance: usize = 0;
    let mut overheads: usize = 0;
    let mut awkwardness: usize = 0;
    let mut comfiness: usize = 0;
    let mut usage = UsageMap::new();

    for symbol in text.chars() {
      let key = self.keyboard.key_for(&symbol);

      match key {
        Some(key) => {
          distance += 1;
          effort += key.effort;
          record_usage_map(&mut usage, key);

          let previous_key = self.previous_key.get();
          let is_a_comfy = self.comfy_combo(previous_key, key);
          
          if is_a_comfy {
            comfiness += 1;
          }

          let (same_hand_penalties, awkwardness_penalty) = self.get_penalties(previous_key, key, is_a_comfy);
          let total_penalties = same_hand_penalties + awkwardness_penalty;

          effort += total_penalties;
          overheads += total_penalties;
          awkwardness += awkwardness_penalty;

          self.previous_key.set(key);

          if effort > effort_limit {
            break;
          }
        },
        None => {},
      }
    }

    Summary { effort, distance, overheads, awkwardness, comfiness, usage }
  }

  fn get_penalties(self: &Self, last_key: &Key, next_key: &Key, is_a_comfy: bool) -> (usize, usize) {
    let same_key = last_key == next_key;
    let changed_row = last_key.coords.0 != 0;
    let is_row_jumping = !same_key && changed_row;

    let mut same_hand_penalties = 0;
    let mut awkwardness_penalty = 0;

    if is_row_jumping && last_key.hand == next_key.hand {
      same_hand_penalties = self.same_hand_penalties(last_key, next_key, is_a_comfy);
      awkwardness_penalty = self.awkward_penalty(last_key, next_key);
    }

    (same_hand_penalties, awkwardness_penalty)
  }

  fn same_hand_penalties(self: &Self, last_key: &Key, next_key: &Key, is_a_comfy: bool) -> usize {
    let mut penalties = SAME_HAND_PENALTY;

    if last_key.finger == next_key.finger {
      penalties += SAME_FINGER_PENALTY;
    }
    
    if !is_a_comfy {
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
      if *coordinate == last_key.coords {
        penalties += BAD_STARTER_PENALTY;
        break;
      }
    }

    penalties
  }

  fn comfy_combo(self: &Self, last_key: &Key, next_key: &Key) -> bool {
    let pair: CoordinatePair = (last_key.coords, next_key.coords);

    self.comfies_map.contains(&pair)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
  );

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
      awkwardness: 0,
      comfiness: 0,
      usage: map! {
        (3, 0) => 1,
        (3, 1) => 1,
        (3, 2) => 1,
        (3, 3) => 1,
        (3, 6) => 1,
        (3, 7) => 1,
        (3, 8) => 1,
        (3, 9) => 1
      }
    })
  }

  #[test]
  fn penalises_same_finger_usage() {
    let penalty = SAME_HAND_PENALTY + SAME_FINGER_PENALTY + ROW_JUMP_PENALTY;

    assert_eq!(run_text("fr"), Summary {
      effort: penalty + 6,
      distance: 2,
      overheads: penalty,
      awkwardness: 0,
      comfiness: 0,
      usage: map! {
        (2, 3) => 1,
        (3, 3) => 1
      }
    })
  }

  #[test]
  fn does_not_penalise_same_key_usage() {
    assert_eq!(run_text("ff"), Summary {
      effort: 0,
      distance: 2,
      overheads: 0,
      awkwardness: 0,
      comfiness: 0,
      usage: map! {
        (2, 3) => 2
      }
    })
  }

  #[test]
  fn penalises_row_jumps() {
    let penalty = SAME_HAND_PENALTY + ROW_JUMP_PENALTY;

    assert_eq!(run_text("at"), Summary {
      effort: penalty + 1 + 11,
      distance: 2,
      overheads: penalty,
      awkwardness: 0,
      comfiness: 0,
      usage: map! {
        (2, 0) => 1,
        (3, 4) => 1
      }
    })
  }

  #[test]
  fn penalises_row_skips() {
    let penalty = SAME_HAND_PENALTY + ROW_SKIP_PENALTY;

    assert_eq!(run_text("vq"), Summary {
      effort: penalty + 6 + 6,
      distance: 2,
      overheads: penalty,
      awkwardness: 0,
      comfiness: 0,
      usage: map! {
        (1, 3) => 1,
        (3, 0) => 1
      }
    })
  }

  #[test]
  fn penalises_bad_starters() {
    let penalty = SAME_HAND_PENALTY + BAD_STARTER_PENALTY;

    assert_eq!(run_text("qw"), Summary {
      effort: penalty + 6 + 2,
      distance: 2,
      overheads: penalty,
      awkwardness: BAD_STARTER_PENALTY,
      comfiness: 0,
      usage: map! {
        (3, 0) => 1,
        (3, 1) => 1
      }
    });
  }

  #[test]
  fn doesnt_penalise_bad_starter_on_hand_switch() {
    assert_eq!(run_text("qi"), Summary {
      effort: 6 + 1,
      distance: 2,
      overheads: 0,
      awkwardness: 0,
      comfiness: 0,
      usage: map! {
        (3, 0) => 1,
        (3, 7) => 1
      }
    });
  }

  #[test]
  fn adds_extra_penalty_on_bad_starters_and_row_jump() {
    let penalty = SAME_HAND_PENALTY + BAD_STARTER_PENALTY + ROW_JUMP_PENALTY;

    assert_eq!(run_text("qs"), Summary {
      effort: penalty + 6 + 0,
      distance: 2,
      overheads: penalty,
      awkwardness: BAD_STARTER_PENALTY,
      comfiness: 0,
      usage: map! {
        (3, 0) => 1,
        (2, 1) => 1
      }
    });
  }

  #[test]
  fn adds_extra_penalty_on_bad_starters_and_skip_jump() {
    let penalty = SAME_HAND_PENALTY + BAD_STARTER_PENALTY + ROW_SKIP_PENALTY;

    assert_eq!(run_text("qv"), Summary {
      effort: penalty + 6 + 6,
      distance: 2,
      overheads: penalty,
      awkwardness: BAD_STARTER_PENALTY,
      comfiness: 0,
      usage: map! {
        (3, 0) => 1,
        (1, 3) => 1
      }
    });
  }

  #[test]
  fn adds_extra_penalty_on_bad_starters_and_same_finger() {
    let penalty = SAME_HAND_PENALTY + BAD_STARTER_PENALTY + ROW_SKIP_PENALTY + SAME_FINGER_PENALTY;

    assert_eq!(run_text("qz"), Summary {
      effort: penalty + 6 + 7,
      distance: 2,
      overheads: penalty,
      awkwardness: BAD_STARTER_PENALTY,
      comfiness: 0,
      usage: map! {
        (3, 0) => 1,
        (1, 0) => 1
      }
    });
  }

  #[test]
  fn doesnt_penalise_comfies_for_row_jumps() {
    let penalty = SAME_HAND_PENALTY + SAME_HAND_PENALTY;

    assert_eq!(run_text("wfli"), Summary {
      effort: penalty + 3,
      distance: 4,
      overheads: penalty,
      awkwardness: 0,
      comfiness: 2,
      usage: map! {
        (2, 8) => 1,
        (2, 3) => 1,
        (3, 7) => 1,
        (3, 1) => 1
      }
    });
  }
}
