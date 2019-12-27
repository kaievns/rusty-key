type Mapping = [[usize; 13]; 4];

const FINGERS: Mapping = [
  [1, 1, 2, 3, 4, 4, 4, 4, 3, 2, 2, 1, 1],
     [1, 2, 3, 4, 4, 4, 4, 3, 2, 1, 1, 1, 1],
     [1, 2, 3, 4, 4, 4, 4, 3, 2, 1, 1, 0, 0],
      [1, 2, 3, 4, 4, 4, 4, 3, 2, 1, 0, 0, 0]
];

const HANDS: Mapping = [
  [0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1],
     [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
     [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
      [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1]
];


const EFFORTS: Mapping = [
  [17, 14, 08, 08, 13, 16, 23, 19, 09, 08, 07, 15, 17],
    [06, 02, 01, 06, 11, 14, 09, 01, 01, 07, 09, 13, 18],
    [01, 00, 00, 00, 07, 07, 00, 00, 00, 01, 05, 0, 0],
      [07, 08, 10, 06, 10, 04, 02, 05, 05, 03, 0, 0, 0]
];

pub const SPACE_EFFORT: usize = 0;
pub const ENTER_EFFORT: usize = 11;
const LEFT_SHIFT_EFFORT: usize = 5;
const RIGHT_SHIFT_EFFORT: usize = 11;


fn find_in_mapping(mapping: Mapping, row: usize, i: usize) -> usize {
  let y = if row < 4 { 4 - row } else { 0 };
  let x = if i < 13 { i } else { 12 };

  mapping[y][x]
}

pub fn effort_for(row: usize, i: usize, shifted: bool) -> usize {
  let mut effort = find_in_mapping(EFFORTS, row, i);
  
  if shifted {
    let hand = find_in_mapping(HANDS, row, i);

    effort += if hand == 0 { RIGHT_SHIFT_EFFORT } else { LEFT_SHIFT_EFFORT };
  }

  effort
}

pub fn hand_and_finger(row: usize, i: usize) -> (bool, usize) {
  let finger = find_in_mapping(FINGERS, row, i);
  let hand = find_in_mapping(HANDS, row, i) == 1;

  (hand, finger)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn gets_right_hands_and_fingers() {
    assert_eq!(hand_and_finger(1, 0), (false, 1));
    assert_eq!(hand_and_finger(1, 1), (false, 2));
    assert_eq!(hand_and_finger(1, 2), (false, 3));
    assert_eq!(hand_and_finger(1, 3), (false, 4));
    assert_eq!(hand_and_finger(1, 4), (false, 4));
    assert_eq!(hand_and_finger(1, 5), (true, 4));
    assert_eq!(hand_and_finger(1, 6), (true, 4));
    assert_eq!(hand_and_finger(1, 7), (true, 3));
    assert_eq!(hand_and_finger(1, 8), (true, 2));
    assert_eq!(hand_and_finger(1, 9), (true, 1));
  }

  #[test]
  fn calculates_normal_efforts_correctly() {
    let efforts = (
      effort_for(1, 0, false),
      effort_for(2, 1, false),
      effort_for(3, 2, false),
      effort_for(4, 3, false),
      effort_for(3, 4, false),
      effort_for(2, 5, false),
      effort_for(1, 6, false),
      effort_for(2, 7, false),
      effort_for(3, 8, false)
    );

    assert_eq!(efforts, (
      7 as usize, 
      0 as usize, 
      1 as usize, 
      8 as usize, 
      11 as usize, 
      7 as usize, 
      2 as usize, 
      0 as usize,
      1 as usize
    ));
  }

  #[test]
  fn calculates_shifted_efforts_correctly() {
    let efforts = (
      effort_for(1, 0, true),
      effort_for(2, 1, true),
      effort_for(3, 2, true),
      effort_for(4, 3, true),
      effort_for(3, 4, true),
      effort_for(2, 5, true),
      effort_for(1, 6, true),
      effort_for(2, 7, true),
      effort_for(3, 8, true)
    );

    assert_eq!(efforts, (
      7 + RIGHT_SHIFT_EFFORT, 
      0 + RIGHT_SHIFT_EFFORT , 
      1 + RIGHT_SHIFT_EFFORT, 
      8 + RIGHT_SHIFT_EFFORT, 
      11 + RIGHT_SHIFT_EFFORT, 
      7 + LEFT_SHIFT_EFFORT, 
      2 + LEFT_SHIFT_EFFORT, 
      0 + LEFT_SHIFT_EFFORT,
      1 + LEFT_SHIFT_EFFORT
    ));    
  }
}