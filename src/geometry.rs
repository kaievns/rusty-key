use crate::parser;
use crate::parser::{Position};
use hashbrown::HashMap;

#[derive(Debug)]
pub struct Geometry {
  template: &'static str,
  fingers: &'static str,
  hands: &'static str,
  efforts: &'static str,
  pub rolling_pairs: &'static str,
  pub bad_starters: &'static str
}

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
                t
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
    vd vw vs va    mk ml m; mo mi ?l
  ",
  bad_starters: "
    q    r t y u     p [ ] \\
       d   g h   k    '
     z x c  b n   , . / 
  "
};

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
       t t t     t t t
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
         t t     t t
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
  Right,
  Thumb // don't treat thumbs as hands so they wouldn't penalised for the same hand actions
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Info {
  pub position: Position,
  pub hand: Hand,
  pub finger: Finger,
  pub effort: usize,
}

pub type SpecialsMapping = HashMap<SpecialSymbol, Info>;

impl Geometry {
  pub fn info_for(self: &Self, position: Position) -> Option<Info> {
    match self.position_in_geometry(position) {
      Some(position) => {
        let hand = self.hand_for(position);
        let finger = self.finger_for(position);
        let effort = self.effort_for(position);

        Some(Info { hand, finger, effort, position })
      },
      _ => None
    }
  }

  pub fn specials_info(self: &Self) -> SpecialsMapping {
    let mut specials: SpecialsMapping = SpecialsMapping::new();

    for symbol in SpecialSymbol::iter() {
      match self.special_info(symbol) {
        Some(info) => specials.insert(symbol, info),
        None if symbol == SpecialSymbol::RightShift =>
          specials.insert(symbol, self.special_info(SpecialSymbol::LeftShift).unwrap()),
        _ => None
      };
    }

    specials
  }

  fn special_info(self: &Self, symbol: SpecialSymbol) -> Option<Info> {
    match parser::position_for(self.template, self.special_symbol_to_string(symbol)) {
      Some(position) => self.info_for(position),
      _ => None
    }
  }

  fn special_symbol_to_string(self: &Self, symbol: SpecialSymbol) -> String {
    let str = match symbol {
      SpecialSymbol::Tab => "⇥",
      SpecialSymbol::Space => "︺",
      SpecialSymbol::Return => "↵",
      SpecialSymbol::LeftShift => "⇧",
      SpecialSymbol::RightShift => "⇪",
      _ => "Noop"
    };

    str.to_string()
  }

  // remaps a standard QUERTY layout position to the geometry template position
  fn position_in_geometry(self: &Self, position_in_querty: Position) -> Option<Position> {
    match self.letter_in_querty(position_in_querty) {
      Some(letter) => parser::position_for(self.template, letter),
      _ => None
    }
  }

  fn letter_in_querty(self: &Self, position_in_querty: Position) -> Option<String> {
    let querty = "
      ` 1 2 3 4 5 6 7 8 9 0 - =
        q w e r t y u i o p [ ] \\
        a s d f g h j k l ; '
         z x c v b n m , . /
    ";

    parser::value_for(querty, position_in_querty)
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
      Some("t") => Hand::Thumb,
      _ => panic!("Unknown hand code")
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  const GEO: Geometry = US_PC_KEYBOARD;

  #[test]
  fn finds_engtries_correctly() {
    assert_eq!(GEO.info_for((4, 2)), Some(
      Info {
        position: (4,5),
        hand: Hand::Left,
        finger: Finger::Middle,
        effort: 12
      }
    ))
  }

  // #[test]
  // fn finds_hands_correctly() {
  //   assert_eq!(GEO.hand_for((1, 0)), Hand::Left);
  //   assert_eq!(GEO.hand_for((1, 1)), Hand::Left);
  //   assert_eq!(GEO.hand_for((1, 2)), Hand::Left);
  //   assert_eq!(GEO.hand_for((1, 3)), Hand::Left);
  //   assert_eq!(GEO.hand_for((1, 4)), Hand::Left);
  //   assert_eq!(GEO.hand_for((1, 5)), Hand::Right);
  //   assert_eq!(GEO.hand_for((1, 6)), Hand::Right);
  //   assert_eq!(GEO.hand_for((1, 7)), Hand::Right);
  //   assert_eq!(GEO.hand_for((1, 8)), Hand::Right);
  //   assert_eq!(GEO.hand_for((1, 9)), Hand::Right);
  // }

  // #[test]
  // fn finds_fingers_correctly() {
  //   assert_eq!(GEO.finger_for((1, 0)), Finger::Pinky);
  //   assert_eq!(GEO.finger_for((1, 1)), Finger::Ring);
  //   assert_eq!(GEO.finger_for((1, 2)), Finger::Middle);
  //   assert_eq!(GEO.finger_for((1, 3)), Finger::Pointy);
  //   assert_eq!(GEO.finger_for((1, 4)), Finger::Pointy);
  //   assert_eq!(GEO.finger_for((1, 5)), Finger::Pointy);
  //   assert_eq!(GEO.finger_for((1, 6)), Finger::Pointy);
  //   assert_eq!(GEO.finger_for((1, 7)), Finger::Middle);
  //   assert_eq!(GEO.finger_for((1, 8)), Finger::Ring);
  //   assert_eq!(GEO.finger_for((1, 9)), Finger::Pinky);
  // }

  // #[test]
  // fn calculates_normal_efforts_correctly() {
  //   assert_eq!(GEO.effort_for((1, 0), false), 7);
  //   assert_eq!(GEO.effort_for((2, 1), false), 0);
  //   assert_eq!(GEO.effort_for((3, 2), false), 1);
  //   assert_eq!(GEO.effort_for((4, 3), false), 8);
  //   assert_eq!(GEO.effort_for((3, 4), false), 11);
  //   assert_eq!(GEO.effort_for((2, 5), false), 7);
  //   assert_eq!(GEO.effort_for((1, 6), false), 2);
  //   assert_eq!(GEO.effort_for((2, 7), false), 0);
  //   assert_eq!(GEO.effort_for((3, 8), false), 1);
  // }

  // #[test]
  // fn calculates_shifted_efforts_correctly() {
  //   assert_eq!(GEO.effort_for((1, 0), true), 7 + GEO.right_shift_effort);
  //   assert_eq!(GEO.effort_for((2, 1), true), 0 + GEO.right_shift_effort);
  //   assert_eq!(GEO.effort_for((3, 2), true), 1 + GEO.right_shift_effort);
  //   assert_eq!(GEO.effort_for((4, 3), true), 8 + GEO.right_shift_effort);
  //   assert_eq!(GEO.effort_for((3, 4), true), 11 + GEO.right_shift_effort);
  //   assert_eq!(GEO.effort_for((2, 5), true), 7 + GEO.left_shift_effort);
  //   assert_eq!(GEO.effort_for((1, 6), true), 2 + GEO.left_shift_effort);
  //   assert_eq!(GEO.effort_for((2, 7), true), 0 + GEO.left_shift_effort);
  //   assert_eq!(GEO.effort_for((3, 8), true), 1 + GEO.left_shift_effort);
  // }
}