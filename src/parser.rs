use hashbrown::HashMap;
use cached::proc_macro::cached;

pub type Position = (usize, usize);
pub type Mapping = HashMap<Position, String>;

pub fn value_for(template: &'static str, position: Position) -> Option<String> {
  match mapping_for(template).get(&position) {
    Some(str) => Some(str.to_string()),
    _ => None
  }
}

pub fn position_for(template: &'static str, value: String) -> Option<Position> {
  let mapping = mapping_for(template);

  mapping.iter().find_map(|(key, val)| if *val == value { Some(*key) } else { None })
}

#[cached]
fn mapping_for(template: &'static str) -> Mapping {
  let mut mapping = Mapping::new();

  for (row, line) in template.trim().lines().enumerate() {
    let chars = line.trim().to_string();
    let stuff = chars.split_whitespace();
  
    for (pos, char) in stuff.enumerate() {
      mapping.insert((row, pos), char.to_string());
    }
  }

  mapping
}

#[cfg(test)]
mod test {
  use super::*;

  const TEMPLATE: &'static str = "
    1   2 3  
      a b c
         v x z
  ";

  #[test]
  fn it_parses_templates_correctly() {
    assert_eq!(value_for(TEMPLATE, (0,1)), Some(String::from("2")));
    assert_eq!(value_for(TEMPLATE, (1,0)), Some(String::from("a")));
    assert_eq!(value_for(TEMPLATE, (2,2)), Some(String::from("z")));
  }

  #[test]
  fn it_returns_none_when_a_position_doesnt_exists() {
    assert_eq!(value_for(TEMPLATE, (22, 22)), None);
  }

  #[test]
  fn it_finds_positions_by_value() {
    assert_eq!(position_for(TEMPLATE, String::from("1")), Some((0,0)));
    assert_eq!(position_for(TEMPLATE, String::from("b")), Some((1,1)));
    assert_eq!(position_for(TEMPLATE, String::from("z")), Some((2,2)));
  }

  #[test]
  fn it_returns_none_when_a_value_doesnt_exists() {
    assert_eq!(position_for(TEMPLATE, String::from("shnt")), None);
  }
}