use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Summary {
  pub effort: usize,
  pub overheads: usize,
  pub awkwardness: usize,
  pub rollingness: usize
}

impl fmt::Display for Summary {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, 
      "effort: {}\noverheads: {}\nawkwardness: {}\nrollingness: {}", 
      self.effort, self.overheads, self.awkwardness, self.rollingness
    )
  }
}

impl Summary {
  pub fn score(self: &Self) -> f64 {
    let positive = self.rollingness * 300;
    let negative = self.effort + self.overheads + self.awkwardness;
    
    (positive as f64) / (negative as f64)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  
  #[test]
  fn test_score() {
    let qwerty = Summary {
      effort: 39440107,
      overheads: 33819770,
      awkwardness: 12985920,
      rollingness: 86229
    };
    let dvorak = Summary {
      effort: 21221795,
      overheads: 17372425,
      awkwardness: 8782080,
      rollingness: 72790
    };
    let workman = Summary {
      effort: 22606098,
      overheads: 19413950,
      awkwardness: 8054640,
      rollingness: 166615
    };
    let halmak = Summary {
      effort: 19627665,
      overheads: 16798430,
      awkwardness: 7105200,
      rollingness: 126447
    };
    assert_eq!(qwerty.score(), 0.2999415728049913);
    assert_eq!(dvorak.score(), 0.46092666586457787);
    assert_eq!(workman.score(), 0.9981989303657768);
    assert_eq!(halmak.score(), 0.8714213533045594);
  }
}