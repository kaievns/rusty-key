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
    let mutator = Mutator::new();

    let mut members = Members::new();
    members.push((*ancestor).clone());

    for i in 0..POPULATION_SIZE-1 {
      let new_member = if i % 2 == 0 {
        mutator.mutate_keys(&members[i])
      } else {
        mutator.mutate_symbols(&members[i])
      };
      members.push(new_member);
    }

    members
  }

  pub fn deviation_for(self: &Self, member: &Layout) -> f64 {
    let original = &self.members[0];
    let mut diffs = 0;

    for (c1, c2) in original.template.chars().zip(member.template.chars()) {
      if c1 != c2 { diffs += 1; }
    }

    (diffs as f64) / (original.template.len() as f64)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_new() {
    let original = QWERTY.clone();
    let population = Population::new(&original);

    assert_eq!(population.members.len(), POPULATION_SIZE);
    assert_eq!(population.members[0].template, original.template);

    assert_ne!(population.members[0].template, population.members[1].template);
    assert_ne!(population.members[1].template, population.members[2].template);
    assert_ne!(population.members[2].template, population.members[3].template);
  }

  #[test]
  fn test_deviation() {
    let original = QWERTY.clone();
    let population = Population::new(&original);

    assert_eq!(population.deviation_for(&population.members[0]), 0.0);
    assert_eq!(population.deviation_for(&population.members[1]), 4.0/(original.template.len() as f64));
    assert_eq!(population.deviation_for(&population.members[2]), 6.0/(original.template.len() as f64));
    assert_eq!(population.deviation_for(&population.members[3]), 10.0/(original.template.len() as f64));
  }
}