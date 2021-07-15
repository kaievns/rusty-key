use rayon::prelude::*;
use once_cell::sync::OnceCell;

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
    let pair = self.successor();
    Generation::new(self.number + 1, &pair.0)
  }

  pub fn successor(self: &Self) -> (Layout, f64) {
    static SUCCESSOR: OnceCell<(Layout, f64)> = OnceCell::new();
    SUCCESSOR.get_or_init(|| {
      let selection = self.create_selection();
      let score = selection.lucky_draw();
      let index = selection.scores.iter()
        .position(|s| s == score).unwrap();

      let layout = self.population.members.get(index).unwrap();

      ((*layout).clone(), self.rating_for(&score))
    }).clone()
  }

  // pub fn best(&self) -> (Layout, f64) {
  //   let selection = self.create_selection();
  // }

  fn rating_for(&self, score: &Score) -> f64 {
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

}