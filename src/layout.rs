use std::fs;
use std::fmt;
use once_cell::sync::Lazy;

use crate::parser::*;

pub const QWERTY: Lazy<Layout> = Lazy::new(|| { 
  Layout::load(&"assets/layouts/qwerty".to_string())
});
pub const DVORAK: Lazy<Layout> = Lazy::new(|| { 
  Layout::load(&"assets/layouts/dvorak".to_string())
});
pub const COLEMAK: Lazy<Layout> = Lazy::new(|| { 
  Layout::load(&"assets/layouts/colemak".to_string())
});
pub const WORKMAN: Lazy<Layout> = Lazy::new(|| { 
  Layout::load(&"assets/layouts/workman".to_string())
});
pub const THE_1: Lazy<Layout> = Lazy::new(|| { 
  Layout::load(&"assets/layouts/the_1".to_string())
});
pub const HALMAK_21: Lazy<Layout> = Lazy::new(|| { 
  Layout::load(&"assets/layouts/halmak_21".to_string())
});

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
  pub fn load(filename: &String) -> Layout {
    let content = fs::read_to_string(filename).unwrap();

    Layout { template: content.to_string() }
  }
  
  pub fn name(self: &Self) -> String {
    let first6 = &self.entries()[13..19];
    let name = first6.iter().fold(String::new(), |name, key| format!("{}{}", name, key.normal));
  
    name.to_uppercase()
  }

  pub fn long_name(&self) -> String {
    let first6 = &self.entries()[13..23];
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
    assert_eq!(QWERTY.name(), "QWERTY");
    assert_eq!(COLEMAK.name(), "QWFPGJ");
    assert_eq!(DVORAK.name(), "',.PYF");
  }

  #[test]
  fn creates_entries() {
    assert_eq!(QWERTY.entries()[13], Entry {
      normal: "q".to_string(),
      shifted: "Q".to_string(),
      position: (1, 0)
    });

    assert_eq!(QWERTY.entries()[27], Entry {
      normal: "s".to_string(),
      shifted: "S".to_string(),
      position: (2, 1)
    });

    assert_eq!(QWERTY.entries()[39], Entry {
      normal: "c".to_string(),
      shifted: "C".to_string(),
      position: (3, 2)
    });
  }

  #[test]
  fn it_prints() {
    let result = format!("{}", QWERTY.clone());

    assert_eq!(result, " ` 1 2 3 4 5 6 7 8 9 0 - =\n   q w e r t y u i o p [ ] \\\n   a s d f g h j k l ; \'\n    z x c v b n m , . /")
  }
}