use hashbrown::HashMap;

pub type Position = (usize, usize);
pub type Mapping = HashMap<Position, String>;
pub type TwoLayerMapping = HashMap<Position, (String, String)>;

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

pub fn mapping_for(template: &'static str) -> Mapping {
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

pub fn two_layer_mapping_for(template: &String) -> TwoLayerMapping {
  let mut mapping = TwoLayerMapping::new();
  let lines: Vec<&str> = template.trim().lines().collect();
  
  for (row, lines) in lines.chunks(2).enumerate() {
    match lines {
      [shifted_line, normals_line] => {
        let normals = line_to_symbols(normals_line);
        let shifted = line_to_symbols(shifted_line);

        for (pos, (up, low)) in shifted.iter().zip(normals).enumerate() {
          mapping.insert((row, pos), (up.to_string(), low.to_string()));
        }
      },
      _ => panic!("unbalanced template lines")
    }
  }

  mapping
}

fn line_to_symbols(line: &str) -> Vec<String> {
  let trimmed = line.trim().to_string();
  let chunks = trimmed.split_whitespace();

  chunks.map(String::from).collect()
}

#[cfg(test)]
mod test {
  use super::*;

  macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::hashbrown::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
  );

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

  #[test]
  fn it_builds_to_layer_mappings_correctly() {
    let template = "
      W L R B Z :
      w l r b z ;
        S H N T (
        s h n t ,
    ".to_string();

    assert_eq!(two_layer_mapping_for(&template), map! {
      (0, 0) => ("W".to_string(), "w".to_string()), 
      (0, 1) => ("L".to_string(), "l".to_string()), 
      (0, 2) => ("R".to_string(), "r".to_string()), 
      (0, 3) => ("B".to_string(), "b".to_string()),
      (0, 4) => ("Z".to_string(), "z".to_string()), 
      (0, 5) => (":".to_string(), ";".to_string()), 
      (1, 0) => ("S".to_string(), "s".to_string()), 
      (1, 1) => ("H".to_string(), "h".to_string()), 
      (1, 2) => ("N".to_string(), "n".to_string()), 
      (1, 3) => ("T".to_string(), "t".to_string()), 
      (1, 4) => ("(".to_string(), ",".to_string()) 
    });
  }
}