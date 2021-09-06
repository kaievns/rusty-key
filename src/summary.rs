use std::fmt;

use crate::config::CONFIG;
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
      rollingness: result.rollingness * 100.0,
      fitness: fitness * 10.0
    }
  }

  pub fn score(self: &Self) -> f64 {
    let weights = &CONFIG.weights;
    let positive = 
      self.rollingness * (weights.rollingness as f64) + 
      self.fitness * (weights.fitness as f64);
    let negative = 
      self.effort * (weights.effort as f64) + 
      self.overheads * (weights.overheads as f64) + 
      self.awkwardness * (weights.awkwardness as f64);
    
    positive * 10.0 / negative
  }
}

#[cfg(test)]
mod test {
  use super::*;
  
  #[test]
  fn test_score() {
    let qwerty = Summary {
      effort: 25.364358927857115,
      overheads: 21.749859480289295,
      awkwardness: 8.351385453605344,
      rollingness: 5.545480152957474,
      fitness: 3.0612244897959187,
    };
    let dvorak = Summary {
      effort: 13.647965647593287,
      overheads: 11.172394211488275,
      awkwardness: 5.647850530759347,
      rollingness: 4.681203543283286,
      fitness: 6.938775510204081
    };
    let workman = Summary {
      effort: 14.022930115721358,
      overheads: 12.019692695933353,
      awkwardness: 4.928389611959803,
      rollingness: 10.7151906630601,
      fitness: 5.7142857142857135
    };
    let halmak = Summary {
      effort: 12.20033608970624,
      overheads: 10.430491941178513,
      awkwardness: 4.371095513530408,
      rollingness: 8.13194318501912,
      fitness: 8.775510204081632
    };
    assert_eq!(qwerty.score(), 1.5517192716779282);
    assert_eq!(dvorak.score(), 3.8138042585401886);
    assert_eq!(workman.score(), 5.30479151040565);
    assert_eq!(halmak.score(), 6.261573684293221);
  }
}