use crate::layout::*;

pub struct Score {
  pub layout: Layout,
  pub score: f64,
  pub deviation: f64
}

pub type Scores = Vec<Score>;

pub struct Selection {
  pub scores: Scores
}

impl Selection {
  pub fn select_successor(self: &Self) -> Layout {
    self.scores[0].layout.clone() // selection thing should be here
  }
}

#[cfg(test)]
mod test {
  use super::*;
  
  #[test]
  fn test_selection() {
    
  }
}