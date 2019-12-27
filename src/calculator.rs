use crate::layout::*;
use crate::config::*;
use std::collections::HashMap;

type EffortsMap = HashMap<String, usize>;

#[derive(Debug)]
pub struct Calculator {
  // layout: &Layout,
  efforts_map: EffortsMap
}

#[derive(Debug)]
pub struct Summary {
  pub effort: usize
}

impl Calculator {
  pub fn from(layout: &Layout) -> Calculator {
    let efforts_map = Self::build_efforts_map(layout);
    Calculator { efforts_map }
  }

  pub fn summary(self: &Self) -> Summary {
    Summary { effort: 0 }
  }

  fn build_efforts_map(layout: &Layout) -> EffortsMap {
    let mut map = EffortsMap::new();

    for key in layout {
      map.insert(key.normal.to_lowercase(), effort_for(key.row, key.pos, false));
      map.insert(key.shifted.to_uppercase(), effort_for(key.row, key.pos, true));
    }

    map.insert(" ".to_string(), SPACE_EFFORT);
    map.insert("\n".to_string(), ENTER_EFFORT);

    map
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::keyboard::*;

  #[test]
  fn builds_an_effort_map() {
    let querty = Keyboard::querty();
    let calculator = Calculator::from(&querty.layout);

    let efforts = (
      calculator.efforts_map.get(&"q".to_string()),
      calculator.efforts_map.get(&"S".to_string()),
      calculator.efforts_map.get(&"c".to_string()),
      calculator.efforts_map.get(&"F".to_string()),
      calculator.efforts_map.get(&"t".to_string()),
      calculator.efforts_map.get(&"^".to_string()),
      calculator.efforts_map.get(&"y".to_string()),
      calculator.efforts_map.get(&"J".to_string()),
      calculator.efforts_map.get(&"M".to_string())
    );

    assert_eq!(efforts, (
      Some(&6),
      Some(&11),
      Some(&10),
      Some(&11),
      Some(&11),
      Some(&28),
      Some(&14),
      Some(&5),
      Some(&7)
    ))
  }
}