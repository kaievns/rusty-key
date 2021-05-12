pub type Location = (usize, usize);
type Mapping = [[usize; 13]; 4];

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
pub struct Geometry {
  fingers: Mapping,
  hands: Mapping,
  efforts: Mapping,
  pub rolling_pairs: &'static str,
  pub bad_starters: &'static str,
  pub tab_effort: usize,
  pub space_effort: usize,
  pub enter_effort: usize,
  left_shift_effort: usize,
  right_shift_effort: usize
}

pub const US_PC_KEYBOARD: Geometry = Geometry {
  tab_effort: 15,
  space_effort: 0,
  enter_effort: 11,
  left_shift_effort: 5,
  right_shift_effort: 12,
  fingers: [
    [1, 1, 2, 3, 4, 4, 4, 4, 3, 2, 2, 1, 1],
       [1, 2, 3, 4, 4, 4, 4, 3, 2, 1, 1, 1, 1],
       [1, 2, 3, 4, 4, 4, 4, 3, 2, 1, 1, 0, 0],
        [1, 2, 3, 4, 4, 4, 4, 3, 2, 1, 0, 0, 0]
  ],
  hands: [
    [0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1],
       [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
       [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
        [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1]
  ],
  efforts: [
    [17, 14, 08, 08, 13, 16, 23, 19, 09, 08, 07, 15, 17],
      [06, 02, 01, 06, 11, 14, 09, 01, 01, 07, 09, 13, 18],
      [01, 00, 00, 00, 07, 07, 00, 00, 00, 01, 05, 0, 0],
        [07, 08, 10, 06, 10, 04, 02, 05, 05, 03, 0, 0, 0]
  ],
  rolling_pairs: "
    we wf     er ew   re
    as af   sd se sf     df     fe fw fs fa
    vd vw vs va    io    oi oj
    ji jl j; jo   kj   lk li lj lm    ;l ;j
    mk ml m; mo mi ?l
  ",
  bad_starters: "
    q      t y u     p [ ] \\
        d   g h   k    '
      z x c  b n   , . / 
  "
};

fn find_in_mapping(mapping: Mapping, location: Location) -> usize {
  let (row, pos) = location;
  let y = if row < 4 { 4 - row } else { 0 };
  let x = if pos < 13 { pos } else { 12 };

  mapping[y][x]
}

impl Geometry {
  pub fn effort_for(self: &Self, location: Location, shifted: bool) -> usize {
    let mut effort = find_in_mapping(self.efforts, location);

    if shifted {
      let hand = self.hand_for(location);
  
      effort += if hand == Hand::LEFT { self.right_shift_effort } else { self.left_shift_effort };
    }

    effort
  }

  pub fn finger_for(self: &Self, location: Location) -> Finger {
    match find_in_mapping(self.fingers, location) {
      1 => Finger::PINKY,
      2 => Finger::RING,
      3 => Finger::MIDDLE,
      4 => Finger::POINTY,
      5 => Finger::THUMB,
      _ => panic!("Unkown finger code")
    }
  }

  pub fn hand_for(self: &Self, location: Location) -> Hand {
    match find_in_mapping(self.hands, location) {
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

  #[test]
  fn finds_hands_correctly() {
    assert_eq!(GEO.hand_for((1, 0)), Hand::LEFT);
    assert_eq!(GEO.hand_for((1, 1)), Hand::LEFT);
    assert_eq!(GEO.hand_for((1, 2)), Hand::LEFT);
    assert_eq!(GEO.hand_for((1, 3)), Hand::LEFT);
    assert_eq!(GEO.hand_for((1, 4)), Hand::LEFT);
    assert_eq!(GEO.hand_for((1, 5)), Hand::RIGHT);
    assert_eq!(GEO.hand_for((1, 6)), Hand::RIGHT);
    assert_eq!(GEO.hand_for((1, 7)), Hand::RIGHT);
    assert_eq!(GEO.hand_for((1, 8)), Hand::RIGHT);
    assert_eq!(GEO.hand_for((1, 9)), Hand::RIGHT);
  }

  #[test]
  fn finds_fingers_correctly() {
    assert_eq!(GEO.finger_for((1, 0)), Finger::PINKY);
    assert_eq!(GEO.finger_for((1, 1)), Finger::RING);
    assert_eq!(GEO.finger_for((1, 2)), Finger::MIDDLE);
    assert_eq!(GEO.finger_for((1, 3)), Finger::POINTY);
    assert_eq!(GEO.finger_for((1, 4)), Finger::POINTY);
    assert_eq!(GEO.finger_for((1, 5)), Finger::POINTY);
    assert_eq!(GEO.finger_for((1, 6)), Finger::POINTY);
    assert_eq!(GEO.finger_for((1, 7)), Finger::MIDDLE);
    assert_eq!(GEO.finger_for((1, 8)), Finger::RING);
    assert_eq!(GEO.finger_for((1, 9)), Finger::PINKY);
  }

  #[test]
  fn calculates_normal_efforts_correctly() {
    assert_eq!(GEO.effort_for((1, 0), false), 7);
    assert_eq!(GEO.effort_for((2, 1), false), 0);
    assert_eq!(GEO.effort_for((3, 2), false), 1);
    assert_eq!(GEO.effort_for((4, 3), false), 8);
    assert_eq!(GEO.effort_for((3, 4), false), 11);
    assert_eq!(GEO.effort_for((2, 5), false), 7);
    assert_eq!(GEO.effort_for((1, 6), false), 2);
    assert_eq!(GEO.effort_for((2, 7), false), 0);
    assert_eq!(GEO.effort_for((3, 8), false), 1);
  }

  #[test]
  fn calculates_shifted_efforts_correctly() {
    assert_eq!(GEO.effort_for((1, 0), true), 7 + GEO.right_shift_effort);
    assert_eq!(GEO.effort_for((2, 1), true), 0 + GEO.right_shift_effort);
    assert_eq!(GEO.effort_for((3, 2), true), 1 + GEO.right_shift_effort);
    assert_eq!(GEO.effort_for((4, 3), true), 8 + GEO.right_shift_effort);
    assert_eq!(GEO.effort_for((3, 4), true), 11 + GEO.right_shift_effort);
    assert_eq!(GEO.effort_for((2, 5), true), 7 + GEO.left_shift_effort);
    assert_eq!(GEO.effort_for((1, 6), true), 2 + GEO.left_shift_effort);
    assert_eq!(GEO.effort_for((2, 7), true), 0 + GEO.left_shift_effort);
    assert_eq!(GEO.effort_for((3, 8), true), 1 + GEO.left_shift_effort);
  }
}