use rayon::prelude::*;
use once_cell::sync::OnceCell;
use core::cmp::Ordering::Less;

use crate::config::*;
use crate::layout::*;
use crate::keyboard::*;
use crate::population::*;
use crate::selection::*;
use crate::summary::*;

pub struct Generation {
  pub number: usize,
  pub population: Population,

  successor_cache: OnceCell<Layout>,
  best_cache: OnceCell<Layout>,
  selection_cache: OnceCell<Selection>,
  results_cache: OnceCell<Vec<Result>>
}

#[derive(Debug,PartialEq)]
struct Result {
  summary: Summary,
  deviation: f64
}

#[derive(Debug,PartialEq,Clone)]
pub struct Outcome {
  pub winner: Layout,
  pub winner_summary: Summary,
  pub best: Layout,
  pub best_summary: Summary
}

impl Generation {
  pub fn zero() -> Generation {
    Generation::new(1, &QWERTY)
  }

  pub fn new(number: usize, layout: &Layout) -> Generation {
    Generation::spawn(number, layout, layout)
  }

  fn spawn(number: usize, mom: &Layout, dad: &Layout) -> Generation {
    let population = Population::new(mom, dad);

    let successor_cache: OnceCell<Layout> = OnceCell::new();
    let best_cache: OnceCell<Layout> = OnceCell::new();
    let selection_cache: OnceCell<Selection> = OnceCell::new();
    let results_cache: OnceCell<Vec<Result>> = OnceCell::new();

    Generation { 
      number, 
      population,

      successor_cache,
      best_cache,
      selection_cache,
      results_cache
    }
  }

  pub fn next(self: &Self) -> Generation {
    let mom = self.best();
    let dad = self.successor();

    Generation::spawn(self.number + 1, &dad, &mom)
  }

  pub fn successor(self: &Self) -> &Layout {
    self.successor_cache.get_or_init(|| {
      let selection = self.fetch_selection();
      let score = selection.lucky_draw();
      let index = selection.scores.iter()
        .position(|s| s == score).unwrap();

      (*self.population.members.get(index).unwrap()).clone()
    })
  }

  pub fn best(&self) -> &Layout {
    self.best_cache.get_or_init(|| {
      let mut ratings: Vec<(usize, f64)> = self.population.members.iter().enumerate()
        .map(|(i, layout)| (i, self.summary_for(&layout).score())).collect();
      
      ratings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Less));
      let best_rating = *ratings.first().unwrap();

      (*self.population.members.get(best_rating.0).unwrap()).clone()
    })
  }

  pub fn outcome(&self) -> Outcome {
    Outcome {
      winner: self.successor().clone(),
      winner_summary: self.summary_for(self.successor()),
      best: self.best().clone(),
      best_summary: self.summary_for(self.best())
    }
  }

  pub fn summary_for(&self, layout: &Layout) -> Summary {
    let index = self.population.members.iter().position(|l| *l == *layout).unwrap();
    let result = self.calculate_results().get(index).unwrap();

    result.summary.clone()
  }

  fn fetch_selection(&self) -> &Selection {
    self.selection_cache.get_or_init(|| {
      let scores: Vec<Score> = self.calculate_results().iter()
          .map(|result| Score {
            deviation: result.deviation,
            performance: result.summary.score()
          })
          .collect();
        Selection { scores }
    })
  }

  fn calculate_results(&self) -> &Vec<Result> {
    self.results_cache.get_or_init(|| {
      self.population.members.par_iter()
        .map(|layout| self.rate_layout(layout))
        .collect()
    })
  }

  fn rate_layout(self: &Self, layout: &Layout) -> Result {
    let deviation = self.population.deviation_for(layout);
    let keyboard = Keyboard::from(&layout, &CONFIG.geometry);
    let summary = Summary::calculate(&keyboard);

    Result { deviation, summary }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_gen_zero() {
    let generation = Generation::zero();

    assert_eq!(generation.number, 1);
    assert_eq!(generation.population.members[0].name(), "QWERTY");
  }

  #[test]
  fn test_next() {
    let generation = Generation::zero();
    let next_generation = generation.next();

    println!("{:}", next_generation.population.members[0].template);

    assert_eq!(next_generation.number, 2);
    assert_ne!(next_generation.population.members[1].template, generation.population.members[1].template);
  }

  #[test]
  fn test_successor() {
    let generation = Generation::zero();
    let succ1 = generation.successor();
    let succ2 = generation.successor();

    assert_eq!(succ1, succ2);
  }

  #[test]
  fn test_best() {
    let generation = Generation::zero();
    let best1 = generation.best();
    let best2 = generation.best();

    assert_eq!(best1, best2);
  }

  #[test]
  fn test_summary_for() {
    let generation = Generation::zero();
    let layout1 = &generation.population.members[0];
    let layout2 = &generation.population.members[29].clone();

    assert_eq!(generation.summary_for(layout1), generation.summary_for(layout1));
    assert_eq!(generation.summary_for(layout1), generation.summary_for(layout1));
    assert_ne!(generation.summary_for(layout1), generation.summary_for(layout2));
  }
  
  #[test]
  fn test_outcomes() {
    let generation = Generation::zero();
    let outcomes = generation.outcome();

    // QWERTY is rarely the best now
    // assert_eq!(outcomes.best.name(), "QWERTY");
    // assert_eq!(outcomes.best_summary.score(), 4.227205305110984);
  }
}