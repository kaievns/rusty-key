use crate::keyboard::*;

#[derive(Debug)]
pub struct Calculator<'a> {
  keyboard: &'a Keyboard
  // layout: &Layout,
  // efforts_map: EffortsMap
}

#[derive(Debug)]
pub struct Summary {
  pub effort: usize
}

impl Calculator<'_> {
  pub fn from<'a>(keyboard: &'a Keyboard) -> Calculator {
    Calculator { keyboard }
  }

  pub fn summary(self: &Self) -> Summary {
    Summary { effort: 0 }
  }
}

