use hashbrown::HashSet;

use crate::layout::*;
use crate::parser::*;

type Positions = HashSet<Position>;

pub struct Preservative {
  pub restriced_positions: Positions
}

impl Preservative {
  pub fn from(template: String) -> Preservative {
    let restriced_positions = Preservative::parse_positions(template);
    Preservative { restriced_positions }
  }

  pub fn is_safe_position(self: &Self, position: Position) -> bool {
    !self.restriced_positions.contains(&position)
  }

  fn parse_positions(template: String) -> Positions {
    let tmp_layout = Layout { template };
    let mut restriced_positions = Positions::new();
  
    for (position, entry) in tmp_layout.entries().iter().enumerate() {
      let shifted_is_okay = entry.shifted.chars().all(|c|
        c.is_ascii_punctuation() || c.is_ascii_alphanumeric()
      );
      let normal_is_okay = entry.normal.chars().all(|c|
        c.is_ascii_punctuation() || c.is_ascii_alphanumeric()
      );
  
      if shifted_is_okay { restriced_positions.insert((position, 0)); }
      if normal_is_okay { restriced_positions.insert((position, 1)); }
    }
  
    restriced_positions
  }
}

#[cfg(test)]
mod test {
  use super::*;

  macro_rules! set {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_set = HashSet::new();
            $(
                temp_set.insert($x);
            )*
            temp_set
        }
    };
  }

  #[test]
  fn parsing_preserved_positions() {
    let presie = Preservative::from("
      ∙ ! ∙ * ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙
      ∙ 1 ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙
        Q ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙
        q ∙ r ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙
        ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ 
        ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ 
          ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ 
          ∙ ∙ ∙ ∙ ∙ ∙ ∙ , . ∙ 
    ".to_string());
    assert_eq!(presie.restriced_positions, set! [
      (1,0), (1,1),
      (3,0),
      (13, 0), (13, 1),
      (15, 1),
      (44, 1),
      (45, 1)
    ]);
  }
}
