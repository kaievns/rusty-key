use std::fmt;

use hashbrown::HashMap;

use crate::config::*;
use crate::layout::*;
use crate::geometry::*;

pub type KeyMap = HashMap<char, Key>;

#[derive(Debug)]
pub struct Keyboard {
  pub name: String,
  pub layout: Layout,
  pub geometry: Geometry,
  pub key_map: KeyMap
}

impl fmt::Display for Keyboard {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.layout)
  }
}

impl Keyboard {
  pub fn qwerty() -> Keyboard {
    Self::from(QWERTY, DEFAULT_GEOMETRY)
  }

  pub fn from(layout: Layout, geometry: Geometry) -> Keyboard {
    let name = layout.name();
    let keys = Self::keys_from(&layout, &geometry);
    
    Keyboard { name, layout, geometry, key_map: keys }
  }

  pub fn key_for(self: &Self, symbol: &char) -> Option<&Key> {
    self.key_map.get(symbol)
  }

  fn keys_from(layout: &Layout, geometry: &Geometry) -> KeyMap {
    let mut map = KeyMap::new();

    for entry in layout.entries() {
      match geometry.key_for_layout(entry.position) {
        Some(key) => {
          map.insert(entry.normal.chars().next().unwrap(), key);
          map.insert(entry.shifted.chars().next().unwrap(), Key {
            effort: key.effort + geometry.shift_effort_for(&key),
            ..key
          });
        },
        _ => ()
      }
    }

    for (typo, key) in geometry.special_keys().iter() {
      match Keyboard::special_to_char(typo) {
        Some(char) => map.insert(char, *key),
        _ => None
      };
    }

    map
  }

  fn special_to_char(special: &SpecialSymbol) -> Option<char> {
    match special {
      SpecialSymbol::Tab => Some('\t'),
      SpecialSymbol::Space => Some(' '),
      SpecialSymbol::Return => Some('\n'),
      _ => None
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn gets_the_right_name() {
    let keyboard = Keyboard::qwerty();

    assert_eq!(keyboard.name, "QWERTY".to_string());
  }

  #[test]
  fn has_correct_keys_layout() {
    let keyboard = Keyboard::qwerty();

    let first_row = &keyboard.layout.entries()[13..23];
    let letters = first_row.iter().fold(String::new(), |name, key| format!("{}{}", name, key.normal));

    assert_eq!(letters, "qwertyuiop");
  }

  #[test]
  fn builds_correct_key_mapping() {
    let keyboard = Keyboard::qwerty();

    assert_eq!(keyboard.key_map.get(&'q'), Some(&Key { position: (1, 1), hand: Hand::Left, finger: Finger::Pinky, effort: 6 }));
    assert_eq!(keyboard.key_map.get(&'S'), Some(&Key { position: (2, 1), hand: Hand::Left, finger: Finger::Ring, effort: 12 }));
    assert_eq!(keyboard.key_map.get(&'c'), Some(&Key { position: (3, 3), hand: Hand::Left, finger: Finger::Middle, effort: 10 }));
    assert_eq!(keyboard.key_map.get(&'F'), Some(&Key { position: (2, 3), hand: Hand::Left, finger: Finger::Pointy, effort: 12 }));
    assert_eq!(keyboard.key_map.get(&'t'), Some(&Key { position: (1, 5), hand: Hand::Left, finger: Finger::Pointy, effort: 11 }));
    assert_eq!(keyboard.key_map.get(&'^'), Some(&Key { position: (0, 6), hand: Hand::Right, finger: Finger::Pointy, effort: 28 }));
    assert_eq!(keyboard.key_map.get(&'y'), Some(&Key { position: (1, 6), hand: Hand::Right, finger: Finger::Pointy, effort: 14 }));
    assert_eq!(keyboard.key_map.get(&'J'), Some(&Key { position: (2, 6), hand: Hand::Right, finger: Finger::Pointy, effort: 5 }));
    assert_eq!(keyboard.key_map.get(&'M'), Some(&Key { position: (3, 7), hand: Hand::Right, finger: Finger::Pointy, effort: 7 }));
    assert_eq!(keyboard.key_map.get(&' '), Some(&Key { position: (4, 0), hand: Hand::Right, finger: Finger::Thumb, effort: 0 }));
    assert_eq!(keyboard.key_map.get(&'\n'), Some(&Key { position: (2, 11), hand: Hand::Right, finger: Finger::Pinky, effort: 11 }));
    assert_eq!(keyboard.key_map.get(&'\t'), Some(&Key { position: (1, 0), hand: Hand::Left, finger: Finger::Pinky, effort: 15 }));
  }

  #[test]
  fn gives_access_to_keys() {
    let keyboard = Keyboard::qwerty();

    assert_eq!(keyboard.key_for(&'q'), Some(&Key { position: (1, 1), hand: Hand::Left, finger: Finger::Pinky, effort: 6 }));
    assert_eq!(keyboard.key_for(&'S'), Some(&Key { position: (2, 1), hand: Hand::Left, finger: Finger::Ring, effort: 12 }));
  }
}