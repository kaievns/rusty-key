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
  pub location: Location
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
      let location = (key.row, key.pos);

      map.insert(key.normal.to_lowercase().chars().next().unwrap(), Key {
        shifted: false,
        location,
        hand: geometry.hand_for(location),
        finger: geometry.finger_for(location),
        effort: geometry.effort_for(location, false)
      });
      map.insert(key.shifted.to_uppercase().chars().next().unwrap(), Key {
        shifted: true,
        location,
        hand: geometry.hand_for(location),
        finger: geometry.finger_for(location),
        effort: geometry.effort_for(location, true)
      });
    }

    map.insert(' ', Key {
      shifted: false,
      location: (0, 0),
      hand: Hand::LEFT,
      finger: Finger::THUMB,
      effort: geometry.space_effort
    });
    map.insert('\n', Key {
      shifted: false,
      location: (2, 0),
      hand: Hand::RIGHT,
      finger: Finger::PINKY,
      effort: geometry.enter_effort
    });
    map.insert('\t', Key {
      shifted: false,
      location: (3, 0),
      hand: Hand::LEFT,
      finger: Finger::PINKY,
      effort: geometry.tab_effort
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

    assert_eq!(keyboard.key_map.get(&'q'), Some(&Key { location: (3, 0), hand: Hand::LEFT, finger: Finger::PINKY, shifted: false, effort: 6 }));
    assert_eq!(keyboard.key_map.get(&'S'), Some(&Key { location: (2, 1), hand: Hand::LEFT, finger: Finger::RING, shifted: true, effort: 12 }));
    assert_eq!(keyboard.key_map.get(&'c'), Some(&Key { location: (1, 2), hand: Hand::LEFT, finger: Finger::MIDDLE, shifted: false, effort: 10 }));
    assert_eq!(keyboard.key_map.get(&'F'), Some(&Key { location: (2, 3), hand: Hand::LEFT, finger: Finger::POINTY, shifted: true, effort: 12 }));
    assert_eq!(keyboard.key_map.get(&'t'), Some(&Key { location: (3, 4), hand: Hand::LEFT, finger: Finger::POINTY, shifted: false, effort: 11 }));
    assert_eq!(keyboard.key_map.get(&'^'), Some(&Key { location: (4, 6), hand: Hand::RIGHT, finger: Finger::POINTY, shifted: true, effort: 28 }));
    assert_eq!(keyboard.key_map.get(&'y'), Some(&Key { location: (3, 5), hand: Hand::RIGHT, finger: Finger::POINTY, shifted: false, effort: 14 }));
    assert_eq!(keyboard.key_map.get(&'J'), Some(&Key { location: (2, 6), hand: Hand::RIGHT, finger: Finger::POINTY, shifted: true, effort: 5 }));
    assert_eq!(keyboard.key_map.get(&'M'), Some(&Key { location: (1, 6), hand: Hand::RIGHT, finger: Finger::POINTY, shifted: true, effort: 7 }));
    assert_eq!(keyboard.key_map.get(&' '), Some(&Key { location: (0, 0), hand: Hand::LEFT, finger: Finger::THUMB, shifted: false, effort: 0 }));
    assert_eq!(keyboard.key_map.get(&'\n'), Some(&Key { location: (2, 0), hand: Hand::RIGHT, finger: Finger::PINKY, shifted: false, effort: 11 }));
    assert_eq!(keyboard.key_map.get(&'\t'), Some(&Key { location: (3, 0), hand: Hand::LEFT, finger: Finger::PINKY, shifted: false, effort: 15 }));
  }

  #[test]
  fn gives_access_to_keys() {
    let keyboard = Keyboard::querty();

    assert_eq!(keyboard.key_for(&'q'), Some(&Key { location: (3, 0), hand: Hand::LEFT, finger: Finger::PINKY, shifted: false, effort: 6 }));
    assert_eq!(keyboard.key_for(&'S'), Some(&Key { location: (2, 1), hand: Hand::LEFT, finger: Finger::RING, shifted: true, effort: 12 }));
  }
}