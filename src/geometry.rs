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
  pub fn info_for_layout(self: &Self, position: Position) -> Option<Info> {
    match self.layout_to_geometry(position) {
      Some(position) => Some(self.info_for_geometry(position)),
      _ => None
    }
  }

  pub fn info_for_geometry(self: &Self, position: Position) -> Info {
    let hand = self.hand_for(position);
    let finger = self.finger_for(position);
    let effort = self.effort_for(position);

    Info { hand, finger, effort, position }
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
      Some(position) => Some(self.info_for_geometry(position)),
      _ => None
    }
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

  const GEO: Geometry = US_PC_KEYBOARD;

  #[test]
  fn finds_entries_correctly() {
    assert_eq!(GEO.info_for_layout((0, 1)), Some(Info {
      position: (0, 1), hand: Hand::Left, finger: Finger::Pinky, effort: 14
    }));
    assert_eq!(GEO.info_for_layout((1, 2)), Some(Info {
      position: (1, 3), hand: Hand::Left, finger: Finger::Middle, effort: 1
    }));
    assert_eq!(GEO.info_for_layout((2, 5)), Some(Info {
      position: (2, 5), hand: Hand::Right, finger: Finger::Pointy, effort: 7
    }));
    assert_eq!(GEO.info_for_layout((3, 7)), Some(Info {
      position: (3, 8), hand: Hand::Right, finger: Finger::Middle, effort: 5
    }));
  }

  #[test]
  fn returns_none_if_not_found() {
    assert_eq!(GEO.info_for_layout((9,9)), None);
  }

  #[test]
  fn specials_info_check() {
    assert_eq!(GEO.specials_info(), map! {
      SpecialSymbol::Tab        => Info { position: (1, 0),  hand: Hand::Left,  finger: Finger::Pinky, effort: 15 }, 
      SpecialSymbol::Space      => Info { position: (4, 0),  hand: Hand::Right, finger: Finger::Thumb, effort: 0  },
      SpecialSymbol::Return     => Info { position: (2, 11), hand: Hand::Right, finger: Finger::Pinky, effort: 11 }, 
      SpecialSymbol::LeftShift  => Info { position: (3, 0),  hand: Hand::Left,  finger: Finger::Pinky, effort: 5  }, 
      SpecialSymbol::RightShift => Info { position: (3, 11), hand: Hand::Right, finger: Finger::Pinky, effort: 12 }
    });
  }

  #[test]
  fn specials_info_on_full_ortho() {
    assert_eq!(FULL_ORTHO.specials_info(), map! {
      SpecialSymbol::Tab        => Info { position: (4, 5), hand: Hand::Right, finger: Finger::Thumb, effort: 0 }, 
      SpecialSymbol::Space      => Info { position: (4, 2), hand: Hand::Left,  finger: Finger::Thumb, effort: 0 },
      SpecialSymbol::Return     => Info { position: (4, 3), hand: Hand::Right, finger: Finger::Thumb, effort: 0 }, 
      SpecialSymbol::LeftShift  => Info { position: (4, 1), hand: Hand::Left,  finger: Finger::Thumb, effort: 0 }, 
      SpecialSymbol::RightShift => Info { position: (4, 4), hand: Hand::Right, finger: Finger::Thumb, effort: 0 }
    });
  }

  #[test]
  fn specials_info_on_compact_ortho() {
    assert_eq!(COMPACT_ORTHO.specials_info(), map! {
      SpecialSymbol::Tab        => Info { position: (3, 1), hand: Hand::Left,  finger: Finger::Thumb, effort: 0 }, 
      SpecialSymbol::Space      => Info { position: (3, 3), hand: Hand::Right, finger: Finger::Thumb, effort: 0 },
      SpecialSymbol::Return     => Info { position: (3, 2), hand: Hand::Right, finger: Finger::Thumb, effort: 0 }, 
      SpecialSymbol::LeftShift  => Info { position: (3, 0), hand: Hand::Left,  finger: Finger::Thumb, effort: 0 }, 
      SpecialSymbol::RightShift => Info { position: (3, 0), hand: Hand::Left,  finger: Finger::Thumb, effort: 0 }
    });
  }
}