use crate::layout::*;
use crate::config::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Keyboard {
  pub name: String,
  pub layout: Layout,
  pub key_map: KeyMap
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Key {
  pub row: usize,
  pub hand: bool,
  pub finger: usize,
  pub shifted: bool,
  pub effort: usize
}

pub type KeyMap = HashMap<String, Key>;

impl Keyboard {
  pub fn querty() -> Keyboard {
    Self::parse(QUERTY)
  }

  pub fn parse(layout: &str) -> Keyboard {
    Self::from(parse(layout.to_string()))
  }

  pub fn from(layout: Layout) -> Keyboard {
    let name = Self::name_from(&layout);
    let keys = Self::keys_from(&layout);
    
    Keyboard { name, layout, key_map: keys }
  }

  pub fn key_for(self: &Self, symbol: &String) -> Option<&Key> {
    self.key_map.get(symbol)
  }

  fn name_from(layout: &Layout) -> String {
    let first6 = &layout[13..19];
    let name = first6.iter().fold(String::new(), |name, key| format!("{}{}", name, key.normal));

    name.to_uppercase()
  }

  fn keys_from(layout: &Layout) -> KeyMap {
    let mut map = KeyMap::new();

    for key in layout {
      let row = key.row;
      let pos = key.pos;
      let (hand, finger) = hand_and_finger(row, pos);

      map.insert(key.normal.to_lowercase(), Key {
        shifted: false,
        row,
        hand,
        finger,
        effort: effort_for(row, pos, false)
      });
      map.insert(key.shifted.to_uppercase(), Key {
        shifted: true,
        row,
        hand,
        finger,
        effort: effort_for(row, pos, true)
      });
    }

    map.insert(" ".to_string(), Key {
      shifted: false,
      row: 0,
      hand: false,
      finger: 0,
      effort: SPACE_EFFORT
    });
    map.insert("\n".to_string(), Key {
      shifted: false,
      row: 2,
      hand: true,
      finger: 1,
      effort: ENTER_EFFORT
    });
    map.insert("\t".to_string(), Key {
      shifted: false,
      row: 3,
      hand: false,
      finger: 1,
      effort: TAB_EFFORT
    });

    map
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn gets_the_right_name() {
    let keyboard = Keyboard::querty();

    assert_eq!(keyboard.name, "QWERTY".to_string());
  }

  #[test]
  fn has_correct_keys_layout() {
    let keyboard = Keyboard::querty();

    let first_row = &keyboard.layout[13..23];
    let letters = first_row.iter().fold(String::new(), |name, key| format!("{}{}", name, key.normal));

    assert_eq!(letters, "qwertyuiop");
  }

  #[test]
  fn builds_correct_key_mapping() {
    let keyboard = Keyboard::querty();

    let keys = (
      keyboard.key_map.get(&"q".to_string()),
      keyboard.key_map.get(&"S".to_string()),
      keyboard.key_map.get(&"c".to_string()),
      keyboard.key_map.get(&"F".to_string()),
      keyboard.key_map.get(&"t".to_string()),
      keyboard.key_map.get(&"^".to_string()),
      keyboard.key_map.get(&"y".to_string()),
      keyboard.key_map.get(&"J".to_string()),
      keyboard.key_map.get(&"M".to_string()),
      keyboard.key_map.get(&" ".to_string()),
      keyboard.key_map.get(&"\n".to_string()),
      keyboard.key_map.get(&"\t".to_string())
    );

    assert_eq!(keys, (
      Some(&Key { row: 3, hand: false, finger: 1, shifted: false, effort: 6 }), 
      Some(&Key { row: 2, hand: false, finger: 2, shifted: true, effort: 11 }), 
      Some(&Key { row: 1, hand: false, finger: 3, shifted: false, effort: 10 }), 
      Some(&Key { row: 2, hand: false, finger: 4, shifted: true, effort: 11 }), 
      Some(&Key { row: 3, hand: false, finger: 4, shifted: false, effort: 11 }), 
      Some(&Key { row: 4, hand: true, finger: 4, shifted: true, effort: 28 }), 
      Some(&Key { row: 3, hand: true, finger: 4, shifted: false, effort: 14 }), 
      Some(&Key { row: 2, hand: true, finger: 4, shifted: true, effort: 5 }), 
      Some(&Key { row: 1, hand: true, finger: 4, shifted: true, effort: 7 }), 
      Some(&Key { row: 0, hand: false, finger: 0, shifted: false, effort: 0 }), 
      Some(&Key { row: 2, hand: true, finger: 1, shifted: false, effort: 11 }), 
      Some(&Key { row: 3, hand: false, finger: 1, shifted: false, effort: 15 })
    ))
  }

  #[test]
  fn gives_access_to_keys() {
    let keyboard = Keyboard::querty();

    assert_eq!(keyboard.key_for(&"q".to_string()), Some(&Key { row: 3, hand: false, finger: 1, shifted: false, effort: 6 }));
    assert_eq!(keyboard.key_for(&"S".to_string()), Some(&Key { row: 2, hand: false, finger: 2, shifted: true, effort: 11 }));
  }
}