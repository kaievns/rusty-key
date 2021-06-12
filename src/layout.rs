use std::fmt;
use crate::parser::*;

pub const QWERTY: &'static str = "
  ~ ! @ # $ % ^ & * ( ) _ +
  ` 1 2 3 4 5 6 7 8 9 0 - =
    Q W E R T Y U I O P { } |
    q w e r t y u i o p [ ] \\
    A S D F G H J K L : \"
    a s d f g h j k l ; '
      Z X C V B N M < > ?
      z x c v b n m , . /
";

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

// frivolously reinterpreted symbols
// source https://deskthority.net/wiki/BEAKL
pub const BEAKL_15: &'static str = "
  ~ $ # ! @ % ^ < > & { }
    1 2 3 4 5 6 7 8 9 0 - =
    Q H O U X G C R F Z { } |
    q h o u x g c r f z [ ] \\
    Y I E A @ D S T N B \"
    y i e a . d s t n b ;
      J ? ! K ` W M L P V
      j / , k ' w m l p v
";

pub const THE_1: &'static str = "
  ~ | @ # $ % ^ & * ( ) _ +
  ` 1 2 3 4 5 6 7 8 9 0 - =
    K M L U ! V D R \" Q { } <
    k m l u ? v d r ' q [ ] >
    A T H E : C S N O I \\
    a t h e . c s n o i /
      Z P F J ; B G W X Y
      z p f j , b g w x y 
";

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

#[derive(Debug,PartialEq,Clone)]
pub struct Layout {
  pub template: String
}

#[derive(Debug,PartialEq)]
pub struct Entry {
  pub normal: String,
  pub shifted: String,
  pub position: Position
}

impl Layout {
  pub fn name(self: &Self) -> String {
    let first6 = &self.entries()[13..19];
    let name = first6.iter().fold(String::new(), |name, key| format!("{}{}", name, key.normal));
  
    name.to_uppercase()
  }

  pub fn entries(self: &Self) -> Vec<Entry> {
    let mapping = two_layer_mapping_for(&self.template);
    let mut entries: Vec<_> = mapping.into_iter().collect();

    entries.sort_by_key(|a| a.0);

    entries.iter()
      .map(|(position, (shifted, normal))| Entry { 
        position: *position, shifted: shifted.to_string(), normal: normal.to_string()
      })
      .collect() 
  }

  pub fn to_string(self: &Self, us_pc: bool) -> String {
    let mut string = "".to_string();
  
    for (i, key) in self.entries().iter().enumerate() {
      string = format!("{} {}", string, key.normal);
  
      match i {
        12 | 25 => string = format!("{}\n  ", string),
        36 if us_pc => string = format!("{}\n   ", string),

        5 | 17 | 30 | 36 | 41 if !us_pc => string = format!("{}  ", string),
        _ => {}
      }
    }
  
    string
  }  
}

impl fmt::Display for Layout {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_string(true))
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn formats_name() {
    assert_eq!((Layout { template: QWERTY.to_string() }).name(), "QWERTY");
    assert_eq!((Layout { template: COLEMAK.to_string() }).name(), "QWFPGJ");
    assert_eq!((Layout { template: DVORAK.to_string() }).name(), "',.PYF");
  }

  #[test]
  fn creates_entries() {
    assert_eq!((Layout { template: QWERTY.to_string() }).entries()[13], Entry {
      normal: "q".to_string(),
      shifted: "Q".to_string(),
      position: (1, 0)
    });

    assert_eq!((Layout { template: QWERTY.to_string() }).entries()[27], Entry {
      normal: "s".to_string(),
      shifted: "S".to_string(),
      position: (2, 1)
    });

    assert_eq!((Layout { template: QWERTY.to_string() }).entries()[39], Entry {
      normal: "c".to_string(),
      shifted: "C".to_string(),
      position: (3, 2)
    });
  }

  #[test]
  fn it_prints() {
    let result = format!("{}", Layout { template: QWERTY.to_string() });

    assert_eq!(result, " ` 1 2 3 4 5 6 7 8 9 0 - =\n   q w e r t y u i o p [ ] \\\n   a s d f g h j k l ; \'\n    z x c v b n m , . /")
  }
}