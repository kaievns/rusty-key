use crate::keyboard::*;

#[derive(Debug)]
pub struct Calculator<'a> {
  keyboard: &'a Keyboard
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Summary {
  pub effort: usize,
  pub distance: usize
}

impl Calculator<'_> {
  pub fn from<'a>(keyboard: &'a Keyboard) -> Calculator {
    Calculator { keyboard }
  }

  pub fn run(self: &Self, text: &String) -> Summary {
    let mut effort: usize = 0;
    let mut distance: usize = 0;

    for symbol in text.chars() {
      let key = self.keyboard.key_for(&symbol.to_string());

      match key {
        Some(key) => {
          effort += key.effort;
          distance += 1;
        },
        None => {},
      }
    }

    Summary { effort, distance }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn calculates_basic() {
    let text = "Hello world!".to_string();
    let keyboard = Keyboard::querty();
    let calculator = Calculator::from(&keyboard);
    let result = calculator.run(&text);

    assert_eq!(result, Summary {
      effort: 48,
      distance: text.len()
    })
  }
}
