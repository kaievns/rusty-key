use crate::config::*;
use crate::layout::*;
use crate::mutator::*;

type Members = Vec<Layout>;

pub struct Population {
  pub members: Members
}

impl Population {
  pub fn new(ancestor: &Layout) -> Population {
    let members = Population::create_members(ancestor);

    Population { members }
  }

  fn create_members(ancestor: &Layout) -> Members {
    let mutator = Mutator::new(PRESERVED_SYMBOLS);

    let mut members = Members::new();
    members.push((*ancestor).clone());

    for i in 0..POPULATION_SIZE {
      let new_member = if i % 2 == 0 {
        mutator.mutate_keys(&members.get(i).unwrap())
      } else {
        mutator.mutate_symbols(&members.get(i).unwrap())
      };
      members.push(new_member);
    }

    members
  }
}