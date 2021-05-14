use std::fmt;

use hashbrown::HashMap;

use crate::config::*;
use crate::layout::*;
use crate::layouts::{QUERTY};
use crate::geometry::*;

pub type KeyMap = HashMap<char, Key>;

#[derive(Debug)]
pub struct Keyboard {
  pub name: String,
  pub layout: Layout,
  pub geometry: Geometry,
  pub key_map: KeyMap
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Key {
  pub hand: Hand,
  pub finger: Finger,
  pub shifted: bool,
  pub effort: usize,
  pub position: Position
}

impl fmt::Display for Keyboard {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", print(&self.layout))
  }
}

impl Keyboard {
  pub fn querty() -> Keyboard {
    Self::parse(QUERTY)
  }

  pub fn parse(layout: &str) -> Keyboard {
    Self::from(parse(layout.to_string()), DEFAULT_GEOMETRY)
  }

  pub fn from(layout: Layout, geometry: Geometry) -> Keyboard {
    let name = Self::name_from(&layout);
    let keys = Self::keys_from(&layout, &geometry);
    
    Keyboard { name, layout, geometry, key_map: keys }
  }

  pub fn key_for(self: &Self, symbol: &char) -> Option<&Key> {
    self.key_map.get(symbol)
  }

  fn name_from(layout: &Layout) -> String {
    let first6 = &layout[13..19];
    let name = first6.iter().fold(String::new(), |name, key| format!("{}{}", name, key.normal));

    name.to_uppercase()
  }

  fn keys_from(layout: &Layout, geometry: &Geometry) -> KeyMap {
    let mut map = KeyMap::new();

    for key in layout {
      let position = key.position;

      map.insert(key.normal.to_lowercase().chars().next().unwrap(), Key {
        shifted: false,
        position,
        hand: geometry.hand_for(position),
        finger: geometry.finger_for(position),
        effort: geometry.effort_for(position, false)
      });
      map.insert(key.shifted.to_uppercase().chars().next().unwrap(), Key {
        shifted: true,
        position,
        hand: geometry.hand_for(position),
        finger: geometry.finger_for(position),
        effort: geometry.effort_for(position, true)
      });
    }

    map.insert(' ', Key {
      shifted: false,
      position: (0, 0),
      hand: Hand::Left,
      finger: Finger::Thumb,
      effort: 0 //geometry.space_effort
    });
    map.insert('\n', Key {
      shifted: false,
      position: (2, 0),
      hand: Hand::Right,
      finger: Finger::Pinky,
      effort: 0 // geometry.enter_effort
    });
    map.insert('\t', Key {
      shifted: false,
      position: (3, 0),
      hand: Hand::Left,
      finger: Finger::Pinky,
      effort: 0 //geometry.tab_effort
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

    assert_eq!(keyboard.key_map.get(&'q'), Some(&Key { position: (3, 0), hand: Hand::Left, finger: Finger::Pinky, shifted: false, effort: 6 }));
    assert_eq!(keyboard.key_map.get(&'S'), Some(&Key { position: (2, 1), hand: Hand::Left, finger: Finger::Ring, shifted: true, effort: 12 }));
    assert_eq!(keyboard.key_map.get(&'c'), Some(&Key { position: (1, 2), hand: Hand::Left, finger: Finger::Middle, shifted: false, effort: 10 }));
    assert_eq!(keyboard.key_map.get(&'F'), Some(&Key { position: (2, 3), hand: Hand::Left, finger: Finger::Pointy, shifted: true, effort: 12 }));
    assert_eq!(keyboard.key_map.get(&'t'), Some(&Key { position: (3, 4), hand: Hand::Left, finger: Finger::Pointy, shifted: false, effort: 11 }));
    assert_eq!(keyboard.key_map.get(&'^'), Some(&Key { position: (4, 6), hand: Hand::Right, finger: Finger::Pointy, shifted: true, effort: 28 }));
    assert_eq!(keyboard.key_map.get(&'y'), Some(&Key { position: (3, 5), hand: Hand::Right, finger: Finger::Pointy, shifted: false, effort: 14 }));
    assert_eq!(keyboard.key_map.get(&'J'), Some(&Key { position: (2, 6), hand: Hand::Right, finger: Finger::Pointy, shifted: true, effort: 5 }));
    assert_eq!(keyboard.key_map.get(&'M'), Some(&Key { position: (1, 6), hand: Hand::Right, finger: Finger::Pointy, shifted: true, effort: 7 }));
    assert_eq!(keyboard.key_map.get(&' '), Some(&Key { position: (0, 0), hand: Hand::Left, finger: Finger::Thumb, shifted: false, effort: 0 }));
    assert_eq!(keyboard.key_map.get(&'\n'), Some(&Key { position: (2, 0), hand: Hand::Right, finger: Finger::Pinky, shifted: false, effort: 11 }));
    assert_eq!(keyboard.key_map.get(&'\t'), Some(&Key { position: (3, 0), hand: Hand::Left, finger: Finger::Pinky, shifted: false, effort: 15 }));
  }

  #[test]
  fn gives_access_to_keys() {
    let keyboard = Keyboard::querty();

    assert_eq!(keyboard.key_for(&'q'), Some(&Key { position: (3, 0), hand: Hand::Left, finger: Finger::Pinky, shifted: false, effort: 6 }));
    assert_eq!(keyboard.key_for(&'S'), Some(&Key { position: (2, 1), hand: Hand::Left, finger: Finger::Ring, shifted: true, effort: 12 }));
  }
}