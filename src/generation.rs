use rayon::prelude::*;

use crate::config::*;
use crate::layout::*;
use crate::keyboard::*;
use crate::calculator::*;
use crate::population::*;
use crate::selection::*;
use crate::profiler;

pub struct Generation {
  pub number: usize,
  population: Population
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
    let layout = self.select_successor();
    Generation::new(self.number + 1, &layout)
  }

  fn select_successor(self: &Self) -> Layout {
    let scores = self.rate_layouts();
    let selection = Selection { scores };
    let winner_score = selection.select_the_fittest();
    let winner_index = selection.scores.iter()
      .position(|score| score == winner_score).unwrap();

    (*self.population.members.get(winner_index).unwrap()).clone()
  }

  fn rate_layouts(self: &Self) -> Scores {
    self.population.members.par_iter()
      .map(|layout| self.rate_layout(layout))
      .collect()
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