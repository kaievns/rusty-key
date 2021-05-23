use crate::parser;
use crate::parser::{Position};
use hashbrown::{HashMap,HashSet};

#[derive(Debug,PartialEq,Eq)]
pub struct Geometry {
  template: &'static str,
  fingers: &'static str,
  hands: &'static str,
  efforts: &'static str,
  rolling_pairs: &'static str,
  bad_starters: &'static str
}

#[allow(dead_code)]
pub const US_PC_KEYBOARD: Geometry = Geometry {
  template: "
    ` 1 2 3 4 5 6 7 8 9 0 - =
    ⇥ q w e r t y u i o p [ ] \\
      a s d f g h j k l ; ' ↵
    ⇧  z x c v b n m , . /  ⇪
              ︺  
  ",
  hands: "
    l l l l l l r r r r r r r
    l l l l l l r r r r r r r r
      l l l l l r r r r r r r
    l  l l l l l r r r r r  r
                r
  ",
  fingers: "
    1 1 2 3 4 4 4 4 3 2 2 1 1
    1 1 2 3 4 4 4 4 3 2 1 1 1 1
      1 2 3 4 4 4 4 3 2 1 1 1
    1  1 2 3 4 4 4 4 3 2 1  1
                5
  ",
  efforts: "
    17 14 08 08 13 16 23 19 09 08 07 15 17
    15 06 02 01 06 11 14 09 01 01 07 09 13 18
       01 00 00 00 07 07 00 00 00 01 05 11
    05  07 08 10 06 10 04 02 05 05 03   12
                    00
  ",
  rolling_pairs: "
    we wf   er ew   re   io    oi oj
    as af   sd se sf  df  fe fw fs fa   ji jl j; jo   kj  lk li lj lm  ;l ;j
    vd vw vs va    mk ml m; mo mi //l
  ",
  bad_starters: "
    q    r t y u     p [ ] \\
       d   g h   k    '
     z x c  b n   , . / 
  "
};

#[allow(dead_code)]
pub const FULL_ORTHO: Geometry = Geometry {
  template: "
    1 2 3 4 5   6 7 8 9 0 - =
    q w e r t   y u i o p [ ]
    a s d f g   h j k l ; ' \\
    z x c v b   n m , . /
       ` ⇧ ︺    ↵ ⇪ ⇥
  ",
  hands: "
    l l l l l   r r r r r r r
    l l l l l   r r r r r r r
    l l l l l   r r r r r r r
    l l l l l   r r r r r
       l l l     r r r
  ",
  fingers: "
    1 2 3 4 4   4 4 3 2 1 1 1
    1 2 3 4 4   4 4 3 2 1 1 1
    1 2 3 4 4   4 4 3 2 1 1 1
    1 2 3 4 4   4 4 3 2 1
       5 5 5     5 5 5
  ",
  efforts: "
    14 08 07 13 16   16 13 07 08 14 15 17
    07 02 01 06 12   12 06 01 01 07 09 13
    01 00 00 00 07   07 00 00 00 01 05 11
    07 08 10 04 08   08 04 10 08 07
         00 00 00     00 00 00
  ",
  rolling_pairs: "
    we wr wf er ew oi ou oj iu io
    as af ;l ;j sd se sf df li lk lj kj fe fw fs fa j;
  ",
  bad_starters: "
    q     r t   y u     p [ ]
        d   g   h   k     ' \\
    z x c   b   n   , . / 
  "
};

#[allow(dead_code)]
pub const COMPACT_ORTHO: Geometry = Geometry {
  template: "
    q w e r t   y u i o p
    a s d f g   h j k l ;
    z x c v b   n m , . /
         ⇧ ⇥     ↵ ︺
  ",
  hands: "
    l l l l l   r r r r r
    l l l l l   r r r r r
    l l l l l   r r r r r
         l l     r r
  ",
  fingers: "
    1 2 3 4 4   4 4 3 2 1
    1 2 3 4 4   4 4 3 2 1
    1 2 3 4 4   4 4 3 2 1
         5 5     5 5
  ",
  efforts: "
    07 02 01 06 12   12 06 01 01 07
    01 00 00 00 07   07 00 00 00 01
    07 08 10 04 08   08 04 10 08 07
            00 00     00 00
  ",
  rolling_pairs: FULL_ORTHO.rolling_pairs,
  bad_starters: "
    q     r t   y u     p
        d   g   h   k    
    z x c   b   n   , . / 
  "
};

use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

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
  pub fn key_for_layout(self: &Self, position: Position) -> Option<Key> {
    match self.layout_to_geometry(position) {
      Some(position) => Some(self.key_for_geometry(position)),
      _ => None
    }
  }

  pub fn key_for_geometry(self: &Self, position: Position) -> Key {
    let hand = self.hand_for(position);
    let finger = self.finger_for(position);
    let effort = self.effort_for(position);

    Key { hand, finger, effort, position }
  }

  pub fn special_keys(self: &Self) -> SpecialsMapping {
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

  pub fn shift_effort_for(self: &Self, key: &Key) -> usize {
    match key.hand {
      Hand::Left => self.special_effort(&SpecialSymbol::RightShift),
      Hand::Right => self.special_effort(&SpecialSymbol::LeftShift)
    }
  }

  pub fn bad_starting_positions(self: &Self) -> HashSet<Position> {
    let mut positions = HashSet::new();

    for symbol in self.bad_starters.trim().split_whitespace() {
      let char = symbol.chars().next().unwrap();
      positions.insert(parser::position_for(self.template, char.to_string()).unwrap());
    }

    positions
  }

  pub fn rolling_position_pairs(self: &Self) -> HashSet<(Position, Position)> {
    let mut pairs = HashSet::new();

    for pair in self.rolling_pairs.trim().split_whitespace() {
      let mut chars = pair.chars();

      let first_letter = chars.next().unwrap();
      let second_letter = chars.next().unwrap();

      let first_position = parser::position_for(self.template, first_letter.to_string()).unwrap();
      let second_position = parser::position_for(self.template, second_letter.to_string()).unwrap();

      pairs.insert((first_position, second_position));
    }

    pairs
  }

  fn special_key(self: &Self, symbol: SpecialSymbol) -> Option<Key> {
    match parser::position_for(self.template, self.special_symbol_to_string(symbol)) {
      Some(position) => Some(self.key_for_geometry(position)),
      _ => None
    }
  }

  fn special_effort(self: &Self, symbol: &SpecialSymbol) -> usize {
    self.special_key(*symbol).unwrap().effort
  }

  fn special_symbol_to_string(self: &Self, symbol: SpecialSymbol) -> String {
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
  fn layout_to_geometry(self: &Self, position_in_querty: Position) -> Option<Position> {
    let querty = "
      ` 1 2 3 4 5 6 7 8 9 0 - =
        q w e r t y u i o p [ ] \\
        a s d f g h j k l ; '
         z x c v b n m , . /
    ";

    match parser::value_for(querty, position_in_querty) {
      Some(letter) => parser::position_for(self.template, letter),
      _ => None
    }
  }

  fn effort_for(self: &Self, position: Position) -> usize {
    match parser::value_for(self.efforts, position) {
      Some(value) => value.parse().unwrap(),
      _ => panic!("Cannot find the effort mapping")
    }
  }

  fn finger_for(self: &Self, position: Position) -> Finger {
    match parser::value_for(self.fingers, position).as_ref().map(String::as_str) {
      Some("1") => Finger::Pinky,
      Some("2") => Finger::Ring,
      Some("3") => Finger::Middle,
      Some("4") => Finger::Pointy,
      Some("5") => Finger::Thumb,
      _ => panic!("Unkown finger code")
    }
  }

  fn hand_for(self: &Self, position: Position) -> Hand {
    match parser::value_for(self.hands, position).as_ref().map(String::as_str) {
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


  const GEO: Geometry = US_PC_KEYBOARD;

  #[test]
  fn finds_entries_correctly() {
    assert_eq!(GEO.key_for_layout((0, 1)), Some(Key {
      position: (0, 1), hand: Hand::Left, finger: Finger::Pinky, effort: 14
    }));
    assert_eq!(GEO.key_for_layout((1, 2)), Some(Key {
      position: (1, 3), hand: Hand::Left, finger: Finger::Middle, effort: 1
    }));
    assert_eq!(GEO.key_for_layout((2, 5)), Some(Key {
      position: (2, 5), hand: Hand::Right, finger: Finger::Pointy, effort: 7
    }));
    assert_eq!(GEO.key_for_layout((3, 7)), Some(Key {
      position: (3, 8), hand: Hand::Right, finger: Finger::Middle, effort: 5
    }));
  }

  #[test]
  fn returns_none_if_not_found() {
    assert_eq!(GEO.key_for_layout((9,9)), None);
  }

  #[test]
  fn special_keys_check() {
    assert_eq!(GEO.special_keys(), map! {
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
  fn special_keys_on_compact_ortho() {
    assert_eq!(COMPACT_ORTHO.special_keys(), map! {
      SpecialSymbol::Tab        => Key { position: (3, 1), hand: Hand::Left,  finger: Finger::Thumb, effort: 0 }, 
      SpecialSymbol::Space      => Key { position: (3, 3), hand: Hand::Right, finger: Finger::Thumb, effort: 0 },
      SpecialSymbol::Return     => Key { position: (3, 2), hand: Hand::Right, finger: Finger::Thumb, effort: 0 }, 
      SpecialSymbol::LeftShift  => Key { position: (3, 0), hand: Hand::Left,  finger: Finger::Thumb, effort: 0 }, 
      SpecialSymbol::RightShift => Key { position: (3, 0), hand: Hand::Left,  finger: Finger::Thumb, effort: 0 }
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