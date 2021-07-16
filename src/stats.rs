/**
 * Represents stats of an evolution process
 *
 * think about it this way, the evolution does the ticking
 * it's a controller, this does results representation, it's a model
 */
use core::cmp::Ordering::Less;

use crate::layout::*;
use crate::evolution::*;
use crate::generation::*;

type Layouts = Vec<Layout>;

pub struct Stats<'a> {
  evolution: &'a Evolution
}

impl Stats<'_> {
  pub fn new(evolution: &Evolution) -> Stats {
    Stats { evolution }
  }

  pub fn winners(&self) -> Layouts {
    match self.evolution.past_generations.lock() {
      Ok(generations) => generations.iter().map(|g| 
        (*g.successor()).clone()
      ).collect(),
      _ => vec![]
    }
  }

  pub fn best(&self) -> Layouts {
    match self.evolution.past_generations.lock() {
      Ok(generations) => generations.iter().map(|generation| {
        let members = &generation.population.members;
        let mut ratings: Vec<(usize, usize)> = members.iter().enumerate()
          .map(|(i, layout)| (i, self.result_to_score(generation.result_for(layout))))
          .collect();
          
        ratings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Less));
        let best_rating = *ratings.first().unwrap();

        (*members.get(best_rating.0).unwrap()).clone()
      }).collect(),
      _ => vec![]
    }
  }

  pub fn score_for(&self, layout: &Layout) -> Option<usize> {
    match self.result_for(layout) {
      Some(result) => Some(self.result_to_score(&result)),
      _ => None
    }
  }

  fn result_to_score(&self, result: &Result) -> usize {
    (result.summary.score() * result.fitness * 100000.0) as usize
  }

  pub fn result_for(&self, layout: &Layout) -> Option<&Result> {
    match self.evolution.past_generations.lock() {
      Ok(generations) => {
        match generations.iter().find(|g| g.population.members.contains(layout)) {
          Some(gen) => Some(gen.result_for(layout)),
          _ => None
        }
      },
      _ => None
    }
  }
}


#[cfg(test)]
mod test {
  use super::*;
  use std::time;

  #[test]
  fn test_winners() {
    let evolution = Evolution::new();
    let stats = Stats::new(&evolution);

    assert_eq!(stats.winners().len(), 0);

    evolution.start();
    std::thread::sleep(time::Duration::from_millis(600));
    evolution.stop();

    assert!(stats.winners().len() > 0);
  }

  #[test]
  fn test_result_for() {
    let evolution = Evolution::new();
    let stats = Stats::new(&evolution);
    evolution.start();
    std::thread::sleep(time::Duration::from_millis(600));
    evolution.stop();

    let winners = stats.winners();
    let layout = winners.first().unwrap();
    let result = stats.result_for(layout);

    assert!(result != None);
  }

  #[test]
  fn test_score_for() {
    let evolution = Evolution::new();
    let stats = Stats::new(&evolution);
    evolution.start();
    std::thread::sleep(time::Duration::from_millis(600));
    evolution.stop();

    let winners = stats.winners();
    let layout = winners.first().unwrap();
    let score = stats.score_for(layout).unwrap();

    assert!(score > 100);
  }

  #[test]
  fn test_best() {
    let evolution = Evolution::new();
    let stats = Stats::new(&evolution);
    evolution.start();
    std::thread::sleep(time::Duration::from_millis(600));
    evolution.stop();

    assert!(stats.best().len() > 0);
  }
}