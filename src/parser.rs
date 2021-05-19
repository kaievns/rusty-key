// use state::Storage;
use hashbrown::HashMap;
use cached::proc_macro::cached;

pub type Position = (usize, usize);

type Mapping = HashMap<Position, String>;
type MappingCache = HashMap<&'static str, Mapping>;

pub struct TemplateParser {}

pub const TEMPLATES_MAPPER: TemplateParser = TemplateParser{};

impl TemplateParser {
  pub fn value_for(self: &Self, template: &'static str, position: Position) -> Option<String> {
    match mapping_for(template).get(&position) {
      Some(str) => Some(str.to_string()),
      _ => None
    }
  }

  pub fn position_for(self: &Self, template: &'static str, value: String) -> Option<Position> {
    let mapping = mapping_for(template);

    mapping.iter().find_map(|(key, val)| if *val == value { Some(*key) } else { None })
  }

  pub fn all_positions_for(self: &Self, template: &'static str, value: String) -> Vec<Position> {
    let mut list: Vec<Position> = mapping_for(template).iter()
      .filter(|(key, val)| **val == value)
      .map(|(key,val)| *key)
      .collect();
    
    list.sort();
    list
  }
}

#[cached]
fn mapping_for(template: &'static str) -> Mapping {
  let mut mapping = Mapping::new();
  let size = template.trim().lines().count();

  for (i, line) in template.trim().lines().enumerate() {
    let row = size - i; // as in keyboard row from the bottom
    let chars = line.trim().to_string();
    let stuff = chars.split_whitespace();
  
    for (pos, char) in stuff.enumerate() {
      mapping.insert((row, pos + 1), char.to_string());
    }
  }

  mapping
}

#[cfg(test)]
mod test {
  use super::*;

  const M: TemplateParser = TEMPLATES_MAPPER;
  const template: &'static str = "
    1   2 3  
      a b c
         v x z
  ";

  #[test]
  fn it_parses_templates_correctly() {
    assert_eq!(M.value_for(template, (1,2)), Some(String::from("x")));
    assert_eq!(M.value_for(template, (2,1)), Some(String::from("a")));
    assert_eq!(M.value_for(template, (3,3)), Some(String::from("3")));
  }

  #[test]
  fn it_returns_none_when_a_position_doesnt_exists() {
    assert_eq!(M.value_for(template, (22, 22)), None);
  }

  #[test]
  fn it_finds_positions_by_value() {
    assert_eq!(M.position_for(template, String::from("1")), Some((3,1)));
    assert_eq!(M.position_for(template, String::from("b")), Some((2,2)));
    assert_eq!(M.position_for(template, String::from("z")), Some((1,3)));
  }

  #[test]
  fn it_returns_none_when_a_value_doesnt_exists() {
    assert_eq!(M.position_for(template, String::from("shnt")), None);
  }

  #[test]
  fn it_returns_all_known_positions_for_a_value() {
    const template: &'static str = "
     a b c 
       b a
    ";

    assert_eq!(M.all_positions_for(template, String::from("a")), vec![(1, 2), (2, 1)]);
    assert_eq!(M.all_positions_for(template, String::from("b")), vec![(1, 1), (2, 2)]);
    assert_eq!(M.all_positions_for(template, String::from("c")), vec![(2,3)]);
    assert_eq!(M.all_positions_for(template, String::from("d")), vec![]);
  }
}