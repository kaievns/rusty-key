use std::fs;

use toml;
use serde::Deserialize;
use once_cell::sync::Lazy;

use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

use crate::parser;
use crate::parser::{Position};
use hashbrown::{HashMap,HashSet};

pub static US_PC_KEYBOARD: Lazy<Geometry> = Lazy::new(||{ Geometry::load("./assets/geometries/us-pc.toml") });
pub static FULL_ORTHO: Lazy<Geometry> = Lazy::new(||{ Geometry::load("./assets/geometries/full-ortho.toml") });

#[derive(Deserialize,Debug,PartialEq,Eq)]
pub struct Geometry {
  template: String,
  fingers: String,
  hands: String,
  efforts: String,
  rolling_pairs: String,
  bad_starters: String
}

#[derive(EnumIter)]
#[derive(Copy, Clone)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SpecialSymbol {
  Tab,
  Space,
  Return,
  LeftShift,
  RightShift
}

#[derive(Copy, Clone)]
#[derive(Debug, PartialEq)]
pub enum Finger {
  Pinky,
  Ring,
  Middle,
  Pointy,
  Thumb
}

#[derive(Copy, Clone)]
#[derive(Debug, PartialEq)]
pub enum Hand {
  Left,
  Right
}

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Key {
  pub position: Position,
  pub hand: Hand,
  pub finger: Finger,
  pub effort: usize
}

pub type SpecialsMapping = HashMap<SpecialSymbol, Key>;

impl Geometry {
  pub fn load(filename: &str) -> Geometry {
    let data = fs::read_to_string(filename).unwrap();
    toml::from_str(&data).unwrap()
  }

  pub fn key_for_layout(&self, position: Position) -> Option<Key> {
    match self.layout_to_geometry(position) {
      Some(position) => Some(self.key_for_geometry(position)),
      _ => None
    }
  }

  pub fn key_for_geometry(&self, position: Position) -> Key {
    let hand = self.hand_for(position);
    let finger = self.finger_for(position);
    let effort = self.effort_for(position);

    Key { hand, finger, effort, position }
  }

  pub fn special_keys(&self) -> SpecialsMapping {
    let mut specials: SpecialsMapping = SpecialsMapping::new();

    for symbol in SpecialSymbol::iter() {
      match self.special_key(symbol) {
        Some(key) => specials.insert(symbol, key),
        None if symbol == SpecialSymbol::RightShift =>
          specials.insert(symbol, self.special_key(SpecialSymbol::LeftShift).unwrap()),
        _ => None
      };
    }

    specials
  }

  pub fn shift_effort_for(&self, key: &Key) -> usize {
    match key.hand {
      Hand::Left => self.special_effort(&SpecialSymbol::RightShift),
      Hand::Right => self.special_effort(&SpecialSymbol::LeftShift)
    }
  }

  pub fn bad_starting_positions(&self) -> HashSet<Position> {
    let mut positions = HashSet::new();

    for symbol in self.bad_starters.trim().split_whitespace() {
      let char = symbol.chars().next().unwrap();
      positions.insert(parser::position_for(&self.template, char.to_string()).unwrap());
    }

    positions
  }

  pub fn rolling_position_pairs(&self) -> HashSet<(Position, Position)> {
    let mut pairs = HashSet::new();

    for pair in self.rolling_pairs.trim().split_whitespace() {
      let mut chars = pair.chars();

      let first_letter = chars.next().unwrap();
      let second_letter = chars.next().unwrap();

      let first_position = parser::position_for(&self.template, first_letter.to_string()).unwrap();
      let second_position = parser::position_for(&self.template, second_letter.to_string()).unwrap();

      pairs.insert((first_position, second_position));
    }

    pairs
  }

  fn special_key(&self, symbol: SpecialSymbol) -> Option<Key> {
    match parser::position_for(&self.template, self.special_symbol_to_string(symbol)) {
      Some(position) => Some(self.key_for_geometry(position)),
      _ => None
    }
  }

  fn special_effort(&self, symbol: &SpecialSymbol) -> usize {
    self.special_key(*symbol).unwrap().effort
  }

  fn special_symbol_to_string(&self, symbol: SpecialSymbol) -> String {
    let str = match symbol {
      SpecialSymbol::Tab => "⇥",
      SpecialSymbol::Space => "︺",
      SpecialSymbol::Return => "↵",
      SpecialSymbol::LeftShift => "⇧",
      SpecialSymbol::RightShift => "⇪"
    };

    str.to_string()
  }

  // remaps a standard QUERTY layout position to the geometry template position
  fn layout_to_geometry(&self, position_in_querty: Position) -> Option<Position> {
    let querty = "
      ` 1 2 3 4 5 6 7 8 9 0 - =
        q w e r t y u i o p [ ] \\
        a s d f g h j k l ; '
         z x c v b n m , . /
    ".to_string();

    match parser::value_for(&querty, position_in_querty) {
      Some(letter) => parser::position_for(&self.template, letter),
      _ => None
    }
  }

  fn effort_for(&self, position: Position) -> usize {
    match parser::value_for(&self.efforts, position) {
      Some(value) => value.parse().unwrap(),
      _ => panic!("Cannot find the effort mapping")
    }
  }

  fn finger_for(&self, position: Position) -> Finger {
    match parser::value_for(&self.fingers, position).as_ref().map(String::as_str) {
      Some("1") => Finger::Pinky,
      Some("2") => Finger::Ring,
      Some("3") => Finger::Middle,
      Some("4") => Finger::Pointy,
      Some("5") => Finger::Thumb,
      _ => panic!("Unkown finger code")
    }
  }

  fn hand_for(&self, position: Position) -> Hand {
    match parser::value_for(&self.hands, position).as_ref().map(String::as_str) {
      Some("l") => Hand::Left,
      Some("r") => Hand::Right,
      _ => panic!("Unknown hand code")
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::hashbrown::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
  );

  macro_rules! set {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_set = HashSet::new();
            $(
                temp_set.insert($x);
            )*
            temp_set
        }
    };
  }

  #[test]
  fn finds_entries_correctly() {
    assert_eq!(US_PC_KEYBOARD.key_for_layout((0, 1)), Some(Key {
      position: (0, 1), hand: Hand::Left, finger: Finger::Pinky, effort: 14
    }));
    assert_eq!(US_PC_KEYBOARD.key_for_layout((1, 2)), Some(Key {
      position: (1, 3), hand: Hand::Left, finger: Finger::Middle, effort: 1
    }));
    assert_eq!(US_PC_KEYBOARD.key_for_layout((2, 5)), Some(Key {
      position: (2, 5), hand: Hand::Right, finger: Finger::Pointy, effort: 7
    }));
    assert_eq!(US_PC_KEYBOARD.key_for_layout((3, 7)), Some(Key {
      position: (3, 8), hand: Hand::Right, finger: Finger::Middle, effort: 5
    }));
  }

  #[test]
  fn returns_none_if_not_found() {
    assert_eq!(US_PC_KEYBOARD.key_for_layout((9,9)), None);
  }

  #[test]
  fn special_keys_check() {
    assert_eq!(US_PC_KEYBOARD.special_keys(), map! {
      SpecialSymbol::Tab        => Key { position: (1, 0),  hand: Hand::Left,  finger: Finger::Pinky, effort: 15 }, 
      SpecialSymbol::Space      => Key { position: (4, 0),  hand: Hand::Right, finger: Finger::Thumb, effort: 0  },
      SpecialSymbol::Return     => Key { position: (2, 11), hand: Hand::Right, finger: Finger::Pinky, effort: 11 }, 
      SpecialSymbol::LeftShift  => Key { position: (3, 0),  hand: Hand::Left,  finger: Finger::Pinky, effort: 5  }, 
      SpecialSymbol::RightShift => Key { position: (3, 11), hand: Hand::Right, finger: Finger::Pinky, effort: 12 }
    });
  }

  #[test]
  fn special_keys_on_full_ortho() {
    assert_eq!(FULL_ORTHO.special_keys(), map! {
      SpecialSymbol::Tab        => Key { position: (4, 5), hand: Hand::Right, finger: Finger::Thumb, effort: 0 }, 
      SpecialSymbol::Space      => Key { position: (4, 2), hand: Hand::Left,  finger: Finger::Thumb, effort: 0 },
      SpecialSymbol::Return     => Key { position: (4, 3), hand: Hand::Right, finger: Finger::Thumb, effort: 0 }, 
      SpecialSymbol::LeftShift  => Key { position: (4, 1), hand: Hand::Left,  finger: Finger::Thumb, effort: 0 }, 
      SpecialSymbol::RightShift => Key { position: (4, 4), hand: Hand::Right, finger: Finger::Thumb, effort: 0 }
    });
  }

  #[test]
  fn calculate_bad_startes() {
    assert_eq!(US_PC_KEYBOARD.bad_starting_positions(), set! [
      (1, 1), 
      (1, 4), 
      (1, 5), 
      (1, 6), 
      (1, 7), 
      (1, 10), 
      (1, 11), 
      (1, 12), 
      (1, 13), 
      (2, 2), 
      (2, 4), 
      (2, 5), 
      (2, 7), 
      (2, 10),
      (3, 1), 
      (3, 2), 
      (3, 3), 
      (3, 5), 
      (3, 6), 
      (3, 8), 
      (3, 9), 
      (3, 10)
    ]);
  }

  #[test]
  fn calculate_rolling_pairs() {
    assert_eq!(US_PC_KEYBOARD.rolling_position_pairs(), set! [
      ((2, 6), (2, 9)), 
      ((2, 0), (2, 3)), 
      ((2, 3), (1, 2)), 
      ((2, 0), (2, 1)), 
      ((3, 10), (3, 10)), 
      ((2, 1), (2, 2)), 
      ((2, 2), (2, 3)), 
      ((2, 9), (2, 8)), 
      ((2, 6), (1, 9)), 
      ((1, 2), (1, 3)), 
      ((2, 8), (3, 7)), 
      ((3, 7), (1, 8)), 
      ((1, 8), (1, 9)), 
      ((3, 7), (2, 8)), 
      ((3, 4), (2, 0)), 
      ((2, 8), (2, 6)), 
      ((2, 8), (2, 7)), 
      ((3, 7), (1, 9)), 
      ((1, 9), (1, 8)), 
      ((2, 3), (1, 3)), 
      ((2, 1), (2, 3)), 
      ((1, 2), (2, 3)), 
      ((1, 9), (2, 6)), 
      ((3, 7), (2, 9)), 
      ((3, 4), (2, 1)), 
      ((2, 9), (2, 6)), 
      ((1, 3), (1, 2)), 
      ((3, 4), (2, 2)), 
      ((2, 7), (2, 6)), 
      ((3, 4), (1, 2)), 
      ((2, 8), (1, 8)), 
      ((2, 1), (1, 3)), 
      ((2, 6), (2, 8)), 
      ((2, 3), (2, 1)), 
      ((1, 3), (1, 4)), 
      ((2, 3), (2, 0)), 
      ((2, 6), (1, 8)), 
      ((3, 7), (2, 7)), 
      ((1, 4), (1, 3))
    ]);
  }
}