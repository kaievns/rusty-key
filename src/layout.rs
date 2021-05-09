#[derive(Debug)]
pub struct Entry {
  pub normal: String,
  pub shifted: String,
  pub row: usize,
  pub pos: usize
}

pub type Layout = Vec<Entry>;

#[allow(dead_code)]
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

#[allow(dead_code)]
pub const DVORAK: &'static str = "
  ~ ! @ # $ % ^ & * ( ) { } 
  ` 1 2 3 4 5 6 7 8 9 0 [ ]
   \" < > P Y F G C R L ? + |
    ' , . p y f g c r l / = \\
    A O E U I D H T N S _
    a o e u i d h t n s -
      : Q J K X B M W V Z
      ; q j k x b m w v z
";

#[allow(dead_code)]
pub const COLEMAK: &'static str = "
  ~ ! @ # $ % ^ & * ( ) _ +
  ` 1 2 3 4 5 6 7 8 9 0 - =
    Q W F P G J L U Y : { } |
    q w f p g j l u y ; [ ] \\
    A R S T D H N E I O \"
    a r s t d h n e i o '
      Z X C V B K M < > ?
      z x c v b k m , . /
";

#[allow(dead_code)]
pub const WORKMAN: &'static str = "
  ~ ! @ # $ % ^ & * ( ) _ +
  ` 1 2 3 4 5 6 7 8 9 0 - =
    Q D R W B J F U P : { } |
    q d r w b j f u p ; [ ] \\
    A S H T G Y N E O I \"
    a s h t g y n e o i '
      Z X M C V K L < > ?
      z x m c v k l , . /
";

#[allow(dead_code)]
pub const THE_1: &'static str = "
  ~ / @ # $ % ^ & * ( ) _ +
  ` 1 2 3 4 5 6 7 8 9 0 - =
    K M L U ! V D R ' Q { } |
    k m l u ? v d r \" q [ ] \\
    A T H E : C S N O I >
    a t h e . c s n o i <
      Z P F J ; B G W X Y
      z p f j , b g w x y 
";

#[allow(dead_code)]
pub const HALMAK_21: &'static str = "
  ~ ! @ # $ % ^ & * < > _ +
  ` 1 2 3 4 5 6 7 8 9 0 - =
    W L R B Z : Q U D J { } |
    w l r b z ; q u d j [ ] \\
    S H N T ( ) A E O I \"
    s h n t , . a e o i '
      F M V C ? G P X K Y
      f m v c / g p x k y
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

      for (pos, (up, low)) in ups.zip(lows).enumerate() {
        let key = Entry { normal: low.to_string(), shifted: up.to_string(), row, pos };

        keys.push(key);
      }
    }
  }

  keys
}

pub fn print(layout: &Layout) -> String {
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

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_parses() {
    let layout = parse(QUERTY.to_string());

    assert_eq!(format!("{:?}", layout[13]), format!("{:?}", Entry {
      normal: "q".to_string(),
      shifted: "Q".to_string(),
      row: 3,
      pos: 0
    }));

    assert_eq!(format!("{:?}", layout[27]), format!("{:?}", Entry {
      normal: "s".to_string(),
      shifted: "S".to_string(),
      row: 2,
      pos: 1
    }));

    assert_eq!(format!("{:?}", layout[39]), format!("{:?}", Entry {
      normal: "c".to_string(),
      shifted: "C".to_string(),
      row: 1,
      pos: 2
    }));
  }

  #[test]
  fn it_prints() {
    let layout = parse(QUERTY.to_string());
    let result = print(&layout);

    println!("{}", result);

    assert_eq!(result, " ` 1 2 3 4 5 6 7 8 9 0 - =\n   q w e r t y u i o p [ ] \\\n   a s d f g h j k l ; \'\n    z x c v b n m , . /")
  }
}