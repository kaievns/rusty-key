use crate::config::*;
use crate::layout::*;
use crate::mutator::*;

type Members = Vec<Layout>;

pub struct Population {
  pub members: Members
}

impl Population {
  pub fn new(mom: &Layout, dad: &Layout) -> Population {
    let members = Population::create_members(mom, dad);

    Population { members }
  }

  fn create_members(mom: &Layout, dad: &Layout) -> Members {
    let mutator = Mutator::new(PRESERVED_SYMBOLS);
    let offspring = (*mom).clone(); // splicer::splice(mom, dad);
    let mut members = Members::new();

    for i in 0..POPULATION_SIZE {
      members.push(Population::create_sibling(&mutator, &offspring, i % 3));
    }

    members.push((*mom).clone());
    members.push((*dad).clone());
    members.push(offspring);

    members
  }

  fn create_sibling(mutator: &Mutator, layout: &Layout, mutate_times: usize) -> Layout {
    let mut new_layout = (*layout).clone();

    for i in 0..mutate_times {
      new_layout = mutator.mutate_keys(&new_layout);
      if MUTATE_SYMBOLS {
        new_layout = mutator.mutate_symbols(&new_layout);
      }
    }

    new_layout
  }
}