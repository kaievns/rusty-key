use std::fmt;

use crate::keyboard::*;
use crate::calculator;
use crate::profiler;

#[derive(Debug,PartialEq,Clone)]
pub struct Summary {
  pub effort: f64,
  pub overheads: f64,
  pub awkwardness: f64,
  pub rollingness: f64,
  pub fitness: f64
}

impl fmt::Display for Summary  {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, 
      "effort: {}\noverheads: {}\nawkwardness: {}\nrollingness: {}\nfitness: {}", 
      self.effort, self.overheads, self.awkwardness, self.rollingness, self.fitness
    )
  }
}

impl Summary {
  pub fn calculate(keyboard: &Keyboard) -> Summary {
    let result = calculator::process(keyboard);
    let fitness = profiler::calculate_fitness(keyboard);

    Summary {
      effort: result.effort,
      overheads: result.overheads,
      awkwardness: result.awkwardness,
      rollingness: result.rollingness,
      fitness: fitness
    }
  }

  pub fn score(self: &Self) -> f64 {
    let positive = self.rollingness * 30.0 + self.fitness;
    let negative = self.effort + self.overheads + self.awkwardness;
    
    positive / negative
  }
}

#[cfg(test)]
mod test {
  use super::*;
  
  #[test]
  fn test_score() {
    let qwerty = Summary {
      effort: 3944.0107,
      overheads: 3381.9770,
      awkwardness: 1298.5920,
      rollingness: 862.29,
      fitness: 0.12
    };
    let dvorak = Summary {
      effort: 21221.795,
      overheads: 173.72425,
      awkwardness: 87.82080,
      rollingness: 727.90,
      fitness: 0.12
    };
    let workman = Summary {
      effort: 2260.6098,
      overheads: 19413.950,
      awkwardness: 805.4640,
      rollingness: 166.615,
      fitness: 0.12
    };
    let halmak = Summary {
      effort: 1962.7665,
      overheads: 16798.430,
      awkwardness: 710.5200,
      rollingness: 126.447,
      fitness: 0.12
    };
    assert_eq!(qwerty.score(), 2.9994296417714126);
    assert_eq!(dvorak.score(), 1.016467641864655);
    assert_eq!(workman.score(), 0.22235608131340145);
    assert_eq!(halmak.score(), 0.19482257766026942);
  }
}