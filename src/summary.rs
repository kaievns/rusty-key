use std::fmt;
use hashbrown::HashMap;
use crate::parser::{Position};

pub type UsageMap = HashMap<Position, usize>;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Summary {
  pub usage: UsageMap,
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