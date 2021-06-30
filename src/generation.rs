use rayon::prelude::*;

use crate::config::*;
use crate::layout::*;
use crate::keyboard::*;
use crate::calculator::*;
use crate::population::*;
use crate::summary::*;

pub struct Score<'a> {
  layout: &'a Layout,
  summary: Summary,
  deviation: f64
}

type Scores<'a> = Vec<Score<'a>>;

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

  pub fn next(self: &Self, text: &String) -> Generation {
    let layout = self.select_successor(text);
    Generation::new(self.number + 1, &layout)
  }

  fn select_successor(self: &Self, text: &String) -> Layout {
    let scores = self.rate_against(text);
    (*scores[0].layout).clone() // selection thing should be here
  }

  fn rate_against(self: &Self, text: &String) -> Scores {
    self.population.members.par_iter()
      .map(|layout| self.rate_layout_against(layout, text))
      .collect()
  }

  fn rate_layout_against(self: &Self, layout: &Layout, text: &String) -> Score {
    let deviation = self.population.deviation_for(layout);
    let keyboard = Keyboard::from(layout, DEFAULT_GEOMETRY);
    let calculator = Calculator::from(&keyboard);
    let summary = calculator.run(&self.text);

    Score { layout, deviation, summary }
  }
}