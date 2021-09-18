use crate::config::*;
use crate::layout::*;
use crate::mutator::*;

type Members = Vec<Layout>;

pub struct Population {
  pub members: Members
}

impl Population {
  pub fn new(normal: &Layout, elite: &Layout) -> Population {
    let members = Population::create_members(normal, elite);

    Population { members }
  }

  fn create_members(normal: &Layout, elite: &Layout) -> Members {
    let mutator = Mutator::new();

    let mut members = Members::new();

    members.push((*normal).clone()); // retaining the original

    let mut batch = Population::make_prestine_batch(normal, elite);
    
    loop {
      batch = batch.iter().enumerate().map(|(i, layout)| {
        if CONFIG.population.symbols && i % 2 != 0 {
          mutator.mutate_symbols(layout)
        } else {
          mutator.mutate_keys(layout)
        }
      }).collect();

      for layout in batch.iter() { members.push((*layout).clone()); }

      if members.len() > CONFIG.population.size { break; }
    }

    members.truncate(CONFIG.population.size);
    members
  }

  fn make_prestine_batch(normal: &Layout, elite: &Layout) -> Members {
    let batch_size = ((CONFIG.population.size as f64) / (CONFIG.population.steps as f64)).ceil() as usize;
    let elites_per_batch = ((batch_size as f64) * (CONFIG.population.elites as f64) / 100.0).ceil() as usize;
    let normals_per_batch = batch_size - elites_per_batch;

    let mut batch = Members::new();
    
    for i in 0..batch_size {
      let parent = if i < normals_per_batch { normal } else { elite };
      batch.push((*parent).clone());
    }

    batch
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

    assert_eq!(population.members.len(), CONFIG.population.size);
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
    assert_eq!(population.deviation_for(&population.members[2]), 2.0/(original.template.len() as f64));
    assert_eq!(population.deviation_for(&population.members[10]), 2.0/(original.template.len() as f64));
    assert_eq!(population.deviation_for(&population.members[20]), 4.0/(original.template.len() as f64));
    // assert_eq!(population.deviation_for(&population.members[29]), 2.0/(original.template.len() as f64));
  }
}