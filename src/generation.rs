use rayon::prelude::*;
use once_cell::sync::OnceCell;
use core::cmp::Ordering::Less;

use crate::config::*;
use crate::layout::*;
use crate::keyboard::*;
use crate::calculator::*;
use crate::population::*;
use crate::selection::*;
use crate::profiler;

pub struct Generation {
  pub number: usize,
  pub population: Population
}

impl Generation {
  pub fn zero() -> Generation {
    let qwerty = Layout { template: QWERTY.to_string() };
    Generation::new(1, &qwerty)
  }

  pub fn new(number: usize, layout: &Layout) -> Generation {
    let population = Population::new(layout);
    Generation { number, population }
  }

  pub fn next(self: &Self) -> Generation {
    let layout = self.successor();
    Generation::new(self.number + 1, &layout)
  }

  pub fn successor(self: &Self) -> Layout {
    static SUCCESSOR: OnceCell<Layout> = OnceCell::new();
    SUCCESSOR.get_or_init(|| {
      let selection = self.create_selection();
      let score = selection.lucky_draw();
      let index = selection.scores.iter()
        .position(|s| s == score).unwrap();

      let layout = self.population.members.get(index).unwrap();

      (*layout).clone()
    }).clone()
  }

  pub fn best(&self) -> Layout {
    static BEST: OnceCell<Layout> = OnceCell::new();
    BEST.get_or_init(|| {
      let mut ratings: Vec<(usize, f64)> = self.population.members.iter().enumerate()
        .map(|(i, layout)| (i, self.rating_for(&layout))).collect();
      
      ratings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Less));
      let best_rating = *ratings.first().unwrap();

      let layout = self.population.members.get(best_rating.0).unwrap();

      (*layout).clone()
    }).clone()
  }

  pub fn rating_for(&self, layout: &Layout) -> f64 {
    let index = self.population.members.iter().position(|l| *l == *layout).unwrap();
    let score = self.create_selection().scores.get(index).unwrap();

    score.performance * score.fitness
  }

  fn create_selection(&self) -> &'static Selection {
    static SELECTION: OnceCell<Selection> = OnceCell::new();
    SELECTION.get_or_init(|| {
      let scores = self.population.members.par_iter()
          .map(|layout| self.rate_layout(layout))
          .collect();
        Selection { scores }
    })
  }

  fn rate_layout(self: &Self, layout: &Layout) -> Score {
    let deviation = self.population.deviation_for(layout);
    let keyboard = Keyboard::from(&layout, &CONFIG.geometry);
    let calculator = Calculator::from(&keyboard);
    let performance = calculator.run(&CONFIG.data).score();
    let fitness = profiler::calculate_fitness(&keyboard);

    Score { deviation, performance, fitness }
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
    assert_ne!(next_generation.population.members[0].name(), "QWERTY");
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
  fn test_rating_for() {
    let generation = Generation::zero();
    let layout1 = &generation.population.members[0];
    let layout2 = &generation.population.members[6].clone();

    assert_eq!(generation.rating_for(layout1), generation.rating_for(layout1));
    assert_eq!(generation.rating_for(layout1), generation.rating_for(layout1));
    assert_ne!(generation.rating_for(layout1), generation.rating_for(layout2));
  }
}