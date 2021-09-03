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
    let mutator = Mutator::new();

    let mut members = Members::new();

    for i in 0..CONFIG.population_size {
      let mut new_member = if i % 3 == 0 { (*mom).clone() } else { (*dad).clone() };
      let mutate_times = (i as f64 / CONFIG.mutate_every as f64).ceil() as usize;

      for x in 0..mutate_times {
        if CONFIG.mutate_symbols == false || x % 3 != 0 {
          new_member = mutator.mutate_keys(&new_member);
        } else {
          new_member = mutator.mutate_symbols(&new_member);
        } 
      }
      
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
    let population = Population::new(&original, &original);

    assert_eq!(population.members.len(), CONFIG.population_size);
    assert_eq!(population.members[0].template, original.template);

    assert_ne!(population.members[0].template, population.members[1].template);
    assert_ne!(population.members[1].template, population.members[2].template);
    assert_ne!(population.members[2].template, population.members[3].template);
  }

  #[test]
  fn test_deviation() {
    let original = QWERTY.clone();
    let population = Population::new(&original, &original);

    assert_eq!(population.deviation_for(&population.members[0]), 0.0);
    assert_eq!(population.deviation_for(&population.members[1]), 4.0/(original.template.len() as f64));
    assert_eq!(population.deviation_for(&population.members[2]), 4.0/(original.template.len() as f64));
    assert_eq!(population.deviation_for(&population.members[10]), 6.0/(original.template.len() as f64));
    assert_eq!(population.deviation_for(&population.members[20]), 10.0/(original.template.len() as f64));
    assert_eq!(population.deviation_for(&population.members[29]), 12.0/(original.template.len() as f64));
  }
}