#[derive(PartialEq,Debug)]
pub struct Score {
  pub performance: f64,
  pub deviation: f64,
  pub fitness: f64
}

pub type Scores = Vec<Score>;

pub struct Selection {
  pub scores: Scores
}

impl Selection {
  pub fn select_the_fittest(self: &Self) -> &Score {
    let normalised = self.renormalise();
    &self.scores[0] // selection thing should be here
  }

  fn renormalise(self: &Self) -> Scores {
    let max_performance = self.scores.iter().map(|s| s.performance).fold(0./0., f64::max);
    let max_deviation = self.scores.iter().map(|s| s.deviation).fold(0./0., f64::max);
    let max_fitness = self.scores.iter().map(|s| s.fitness).fold(0./0., f64::max);

    self.scores.iter().map(|s| Score {
      performance: s.performance / max_performance,
      deviation: s.deviation / max_deviation,
      fitness: s.fitness / max_fitness
    })
    .collect()
  }
}

#[cfg(test)]
mod test {
  use super::*;

  fn get_scores() -> Scores {
    vec![
      Score { performance: 1.1, deviation: 0.3, fitness: 0.2 },
      Score { performance: 1.2, deviation: 0.5, fitness: 0.4 },
      Score { performance: 1.3, deviation: 0.9, fitness: 0.5 }
    ]
  }
  
  #[test]
  fn test_renormalise() {
    let sel = Selection { scores: get_scores() };

    assert_eq!(sel.renormalise(), vec![
      Score { performance: 0.8461538461538461, deviation: 0.3333333333333333, fitness: 0.4 }, 
      Score { performance: 0.923076923076923, deviation: 0.5555555555555556, fitness: 0.8 }, 
      Score { performance: 1.0, deviation: 1.0, fitness: 1.0 }
    ])
  }
}