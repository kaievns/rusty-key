use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Summary {
  pub effort: usize,
  pub distance: usize,
  pub overheads: usize,
  pub awkwardness: usize,
  pub rollingness: usize
}

impl fmt::Display for Summary {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, 
      "effort: {}\ndistance: {}\noverheads: {}\nawkwardness: {}\nrollingness: {}", 
      self.effort, self.distance, self.overheads, self.awkwardness, self.rollingness
    )
  }
}