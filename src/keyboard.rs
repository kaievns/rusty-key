use crate::layout::*;

#[derive(Debug)]
pub struct Keyboard {
  pub name: String,
  pub layout: Layout
}

impl Keyboard {
  pub fn querty() -> Keyboard {
    Self::parse(QUERTY)
  }

  pub fn parse(layout: &str) -> Keyboard {
    Self::from(parse(layout.to_string()))
  }

  fn from(layout: Layout) -> Keyboard {
    let first6 = &layout[13..19];
    let name = first6.iter().fold(String::new(), |name, key| format!("{}{}", name, key.normal));

    Keyboard { name: name.to_uppercase(), layout }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn gets_the_right_name() {
    let keyboard = Keyboard::querty();

    assert_eq!(keyboard.name, "QWERTY".to_string());
  }

  #[test]
  fn has_correct_keys_layout() {
    let keyboard = Keyboard::querty();

    let first_row = &keyboard.layout[13..23];
    let letters = first_row.iter().fold(String::new(), |name, key| format!("{}{}", name, key.normal));

    assert_eq!(letters, "qwertyuiop");
  }
}