use rand::Rng;

use hashbrown::HashSet;

use crate::dna::*;
use crate::layout::*;
use crate::parser::*;
use crate::preservative::*;

pub struct Mutator {
  presie: Preservative,
  cache: HashSet<String>
}

impl Mutator {
  pub fn new(preserve: &'static str) -> Mutator {
    let presie = Preservative::from(preserve);
    Mutator { presie, cache: HashSet::new() }
  }

  pub fn mutate_keys(self: &Self, layout: &Layout) -> Layout {
    let dna = DNA::from(layout);
    let new_dna = self.swap_random_keys(&dna);

    new_dna.to_layout()
  }

  pub fn mutate_symbols(self: &Self, layout: &Layout) -> Layout {
    let dna = DNA::from(layout);
    let new_dna = self.swap_random_symbols(&dna);

    new_dna.to_layout()
  }

  fn swap_random_keys(self: &Self, original: &DNA) -> DNA {
    let (first_pos, second_pos) = self.two_random_key_positions(&original);

    original.swap_keys(first_pos, second_pos)
  }

  fn swap_random_symbols(self: &Self, original: &DNA) -> DNA {
    let (first_pos, second_pos) = self.two_random_symbol_positions(&original);

    original.swap_symbols(first_pos, second_pos)
  }

  fn two_random_key_positions(self: &Self, sequence: &DNA) -> (usize, usize) {
    let first_position = self.random_safe_key_position(sequence.len());
    let mut second_position: usize;

    loop {
      second_position = self.random_safe_key_position(sequence.len());
      if second_position != first_position { break; }
    }
  
    (first_position, second_position)
  }

  fn random_safe_key_position(self: &Self, limit: usize) -> usize {
    let mut position: usize;

    loop {
      position = self.random_number(limit);

      let shifted_is_safe = self.presie.is_safe_position((position, 0));
      let normal_is_safe = self.presie.is_safe_position((position, 1));

      if shifted_is_safe && normal_is_safe { break; }
    }

    position
  }

  fn two_random_symbol_positions(self: &Self, sequence: &DNA) -> (Position, Position) {
    let non_alpha_positions = sequence.iter().enumerate()
      .filter(|(_, pair)| pair.0.chars().all(|c| !c.is_ascii_alphabetic()))
      .map(|entry| entry.0)
      .collect::<Vec<usize>>();

    let first_position = self.random_safe_symbol_position(&non_alpha_positions);
    let mut second_position: Position;
  
    loop {
      second_position = self.random_safe_symbol_position(&non_alpha_positions);
      if second_position != first_position { break; }
    }
    
    (first_position, second_position)
  }

  fn random_safe_symbol_position(self: &Self, positions: &Vec<usize>) -> Position {
    let mut position: Position;

    loop {
      let index = self.random_number(positions.len());
      let number = positions.get(index).unwrap();
      let layer = self.random_number(2);

      position = (*number, layer);

      if self.presie.is_safe_position(position) { break; }
    }

    position
  }

  fn random_number(self: &Self, size: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..size)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_mutates_keys() {
    let layout = Layout { template: QWERTY.to_string() };
    let mutator = Mutator::new("");
    let new_layout = mutator.mutate_keys(&layout);

    assert_ne!(new_layout.template, layout.template);
  }
  
  #[test]
  fn it_mutates_symbols() {
    let layout = Layout { template: QWERTY.to_string() };
    let mutator = Mutator::new("");
    let new_layout = mutator.mutate_symbols(&layout);

    assert_ne!(new_layout.template, layout.template);
  }

  #[test]
  fn it_never_mutates_preserved_positions() {
    let mutator = Mutator::new("
      ∙ ! ∙ # ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙
      ∙ 1 ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙
        Q ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙
        q ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙
        ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ 
        ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ 
          ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ 
          ∙ ∙ ∙ ∙ ∙ ∙ ∙ , ∙ ∙ 
    ");
    let mut dna = qwerty_dna();

    for _ in 0..2000 {
      dna = mutator.swap_random_keys(&dna);
      dna = mutator.swap_random_symbols(&dna);
    }

    println!("{:?}", dna.pair_at(1));
    println!("{:?}", dna.pair_at(3));
    println!("{:?}", dna.pair_at(13));
    println!("{:?}", dna.pair_at(15));
    println!("{:?}", dna.pair_at(44));

    assert_eq!(("!".to_string(), "1".to_string()), dna.pair_at(1));
    assert_eq!(("Q".to_string(), "q".to_string()), dna.pair_at(13));
    assert_eq!("#".to_string(), dna.pair_at(3).0);
    assert_eq!(",".to_string(), dna.pair_at(44).1);

    assert_ne!("W".to_string(), dna.pair_at(14).0);
    assert_ne!("w".to_string(), dna.pair_at(14).1);
    assert_ne!("3".to_string(), dna.pair_at(3).1);
    assert_ne!("<".to_string(), dna.pair_at(44).0);
  }

  #[test]
  fn test_random_keys_swap() {
    let mutator = Mutator::new("");
    let original = qwerty_dna();

    let new_dna1 = mutator.swap_random_keys(&original);
    let new_dna2 = mutator.swap_random_keys(&original);
    let new_dna3 = mutator.swap_random_keys(&original);
    let new_dna4 = mutator.swap_random_keys(&original);

    assert_eq!(original, qwerty_dna());
    assert_ne!(new_dna1, original);
    assert_ne!(new_dna2, new_dna1);
    assert_ne!(new_dna3, new_dna2);
    assert_ne!(new_dna4, new_dna3);
  }

  #[test]
  fn swapping_random_symbols() {
    let mutator = Mutator::new("");
    let original = qwerty_dna();
    let new_dna1 = mutator.swap_random_symbols(&original);
    let new_dna2 = mutator.swap_random_symbols(&original);
    let new_dna3 = mutator.swap_random_symbols(&original);
    let new_dna4 = mutator.swap_random_symbols(&original);

    assert_eq!(original, qwerty_dna());
    assert_ne!(new_dna1, original);
    assert_ne!(new_dna2, new_dna1);
    assert_ne!(new_dna3, new_dna2);
    assert_ne!(new_dna4, new_dna3);
  }

  #[test]
  fn getting_two_random_symbol_positions() {
    let mutator = Mutator::new("");
    let sequence = qwerty_dna();

    for _ in 0..10 {
      let (pos1, pos2) = mutator.two_random_symbol_positions(&sequence);
      let entry1 = sequence.pair_at(pos1.0);
      let entry2 = sequence.pair_at(pos2.0);

      assert_ne!(pos1, pos2);

      assert_eq!(true, entry1.0.chars().all(|c| !c.is_ascii_alphabetic()), "{:?}", entry1.0);
      assert_eq!(true, entry1.1.chars().all(|c| !c.is_ascii_alphabetic()), "{:?}", entry1.1);
      assert_eq!(true, entry2.0.chars().all(|c| !c.is_ascii_alphabetic()), "{:?}", entry2.0);
      assert_eq!(true, entry2.1.chars().all(|c| !c.is_ascii_alphabetic()), "{:?}", entry2.1);
    }
  }

  fn qwerty_dna() -> DNA {
    DNA::from(&Layout { template: QWERTY.to_string() })
  }
}