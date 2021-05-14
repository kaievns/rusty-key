pub type Position = (usize, usize);

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Key {
  TAB,
  SPACE,
  RETURN,
  LEFT_SHIFT,
  RIGHT_SHIFT
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Finger {
  PINKY,
  RING,
  MIDDLE,
  POINTY,
  THUMB
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Hand {
  LEFT,
  RIGHT
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Info {
  pub position: Position
  pub hand: Hand
  pub finger: Finger
  pub effort: usize
}

#[derive(Debug)]
pub struct Geometry {
  template: &'static str,
  fingers: &'static str,
  hands: &'static str,
  efforts: &'static str,
  rolling_pairs: &'static str,
  bad_starters: &'static str
}

pub const US_PC_KEYBOARD: Geometry = Geometry {
  template: "
    ` 1 2 3 4 5 6 7 8 9 0 - =
    ⇥ q w e r t y u i o p [ ] \\
      a s d f g h j k l ; ' ↵
    ⇪  z x c v b n m , . /  ⇪
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
       ` ⇪ ︺    ↵ ⇪ ⇥
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
  "
  bad_starters: "
    q     r t   y u     p [ ]
        d   g   h   k     ' \\
    z x c   b   n   , . / 
  "
}

pub const COMPACT_ORTHO: Geometry = Geometry {
  template: "
    q w e r t   y u i o p
    a s d f g   h j k l ;
    z x c v b   n m , . /
         ⇪ ⇥     ↵ ︺
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
  "
  rolling_pairs: FULL_ORTHO.rolling_pairs,
  bad_starters: "
    q     r t   y u     p
        d   g   h   k    
    z x c   b   n   , . / 
  "
}

fn find_in_mapping(mapping: Mapping, position: Position) -> usize {
  let (row, pos) = position;
  let y = if row < 4 { 4 - row } else { 0 };
  let x = if pos < 13 { pos } else { 12 };

  mapping[y][x]
}

impl Geometry {
  pub fn effort_for(self: &Self, position: Position, shifted: bool) -> usize {
    let mut effort = find_in_mapping(self.efforts, position);

    if shifted {
      let hand = self.hand_for(position);
  
      effort += if hand == Hand::LEFT { self.right_shift_effort } else { self.left_shift_effort };
    }

    effort
  }

  pub fn finger_for(self: &Self, position: Position) -> Finger {
    match find_in_mapping(self.fingers, position) {
      1 => Finger::PINKY,
      2 => Finger::RING,
      3 => Finger::MIDDLE,
      4 => Finger::POINTY,
      5 => Finger::THUMB,
      _ => panic!("Unkown finger code")
    }
  }

  pub fn hand_for(self: &Self, position: Position) -> Hand {
    match find_in_mapping(self.hands, position) {
      0 => Hand::LEFT,
      1 => Hand::RIGHT,
      _ => panic!("Unknown hand code")
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  const GEO: Geometry = US_PC_KEYBOARD;

  // #[test]
  // fn finds_hands_correctly() {
  //   assert_eq!(GEO.hand_for((1, 0)), Hand::LEFT);
  //   assert_eq!(GEO.hand_for((1, 1)), Hand::LEFT);
  //   assert_eq!(GEO.hand_for((1, 2)), Hand::LEFT);
  //   assert_eq!(GEO.hand_for((1, 3)), Hand::LEFT);
  //   assert_eq!(GEO.hand_for((1, 4)), Hand::LEFT);
  //   assert_eq!(GEO.hand_for((1, 5)), Hand::RIGHT);
  //   assert_eq!(GEO.hand_for((1, 6)), Hand::RIGHT);
  //   assert_eq!(GEO.hand_for((1, 7)), Hand::RIGHT);
  //   assert_eq!(GEO.hand_for((1, 8)), Hand::RIGHT);
  //   assert_eq!(GEO.hand_for((1, 9)), Hand::RIGHT);
  // }

  // #[test]
  // fn finds_fingers_correctly() {
  //   assert_eq!(GEO.finger_for((1, 0)), Finger::PINKY);
  //   assert_eq!(GEO.finger_for((1, 1)), Finger::RING);
  //   assert_eq!(GEO.finger_for((1, 2)), Finger::MIDDLE);
  //   assert_eq!(GEO.finger_for((1, 3)), Finger::POINTY);
  //   assert_eq!(GEO.finger_for((1, 4)), Finger::POINTY);
  //   assert_eq!(GEO.finger_for((1, 5)), Finger::POINTY);
  //   assert_eq!(GEO.finger_for((1, 6)), Finger::POINTY);
  //   assert_eq!(GEO.finger_for((1, 7)), Finger::MIDDLE);
  //   assert_eq!(GEO.finger_for((1, 8)), Finger::RING);
  //   assert_eq!(GEO.finger_for((1, 9)), Finger::PINKY);
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