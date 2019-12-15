#[derive(Debug)]
pub struct Key {
  pub normal: String,
  pub shifted: String,
  pub row: usize,
  pub finger: usize,
  pub hand: bool
}

pub type Layout = Vec<Key>;

pub const QUERTY: &'static str = "
  ~ ! @ # $ % ^ & * ( ) _ +
  ` 1 2 3 4 5 6 7 8 9 0 - =
    Q W E R T Y U I O P { } |
    q w e r t y u i o p [ ] \\
    A S D F G H J K L : \"
    a s d f g h j k l ; '
      Z X C V B N M < > ?
      z x c v b n m , . /
";

pub fn parse(layout: String) -> Layout {
  let mut keys: Layout = vec![];

  let mut upper_line = "".to_string();
  
  for (i, line) in layout.trim().lines().enumerate() {
    if i % 2 == 0 {
      upper_line = line.trim().to_string();
    } else {
      let lower_line = line.trim().to_string();

      let ups = upper_line.split_whitespace();
      let lows = lower_line.split_whitespace();
      let row = 4 - (i - 1) / 2; // as in keyboard row

      for (i, (up, low)) in ups.zip(lows).enumerate() {
        let (hand, finger) = hand_and_finger(row, i);
        let key = Key { normal: low.to_string(), shifted: up.to_string(), row, hand, finger };

        keys.push(key);
      }
    }
  }

  keys
}

pub fn print(layout: Layout) -> String {
  let mut string = "".to_string();

  for (i, key) in layout.iter().enumerate() {
    string = format!("{} {}", string, key.normal);

    match i {
      12 => string = format!("{}\n  ", string),
      25 => string = format!("{}\n  ", string),
      36 => string = format!("{}\n   ", string),
      _ => {}
    }
  }

  string
}

fn hand_and_finger(row: usize, i: usize) -> (bool, usize) {
  let mut hand = false;
  let mut finger = 0;

  (hand, finger)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_parses() {
    let layout = parse(QUERTY.to_string());

    assert_eq!(format!("{:?}", layout[13]), format!("{:?}", Key {
      normal: "q".to_string(),
      shifted: "Q".to_string(),
      row: 3,
      finger: 0,
      hand: false
    }));
  }

  #[test]
  fn it_prints() {
    let layout = parse(QUERTY.to_string());
    let result = print(layout);

    println!("{}", result);

    assert_eq!(result, " ` 1 2 3 4 5 6 7 8 9 0 - =\n   q w e r t y u i o p [ ] \\\n   a s d f g h j k l ; \'\n    z x c v b n m , . /")
  }
}