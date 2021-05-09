use std::fmt;
use std::collections::HashMap;
use crate::keyboard::{Coordinates};

pub type UsageMap = HashMap<Coordinates, usize>;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Summary {
  pub usage: UsageMap,
  pub effort: usize,
  pub distance: usize,
  pub overheads: usize,
  pub awkwardness: usize,
  pub comfiness: usize
}

impl fmt::Display for Summary {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, 
      "effort: {}\ndistance: {}\noverheads: {}\nawkwardness: {}\ncomfiness: {}", 
      self.effort, self.distance, self.overheads, self.awkwardness, self.comfiness
    )
  }
}