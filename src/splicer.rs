use rand::Rng;

use crate::dna::*;
use crate::layout::*;
use crate::parser::*;
use crate::preservative::*;

pub struct Splicer {
  presie: Preservative
}

const CHUNKS_COUNT: usize = 8;

impl Splicer {
  pub fn new(preserve: &'static str) -> Splicer {
    let presie = Preservative::from(preserve);
    Splicer { presie }
  }

  pub fn sex(self: &Self, mom: &Layout, dad: &Layout) -> Layout {
    let (mom_dna, dad_dna) = self.random_parents(&mom, &dad);

    let offspring = self.blank_dna(&mom_dna);

    let moms_chunks = self.chunk(&mom_dna);
    let dads_chunks = self.chunk(&dad_dna);
    let babe_chunks = self.chunk(&offspring);

    // magic here

    offspring.to_layout()
  }

  fn chunk(self: &Self, dna: &DNA) -> Vec<Sequence> {
    let mut chunks: Vec<Sequence> = vec![];
    let chunk_size = dna.sequence.len() / CHUNKS_COUNT + 1;
    let mut tail = dna.sequence.clone();

    for i in 0..CHUNKS_COUNT {
      if tail.len() > chunk_size {
        let new_tail = tail.split_off(chunk_size);
        let head = tail;
        tail = new_tail;
        chunks.push(head);  
      } else {
        chunks.push(tail.clone());
      } 
    }

    chunks
  }

  fn merge(self: &Self, old: &Sequence, new: &Sequence) -> Sequence {
    let mut merged = Sequence::new();

    for (i, pair) in old.iter().enumerate() {
      if self.can_merge(&pair, &new[i]) {
        let new_shifted = self.merge_symbols(&pair.0, &new[i].0);
        let new_normal = self.merge_symbols(&pair.1, &new[i].1);
  
        merged.push((new_shifted, new_normal));
      } else {
        merged.push(pair.clone());
      }
    }

    merged
  }

  fn can_merge(self: &Self, old: &Pair, new: &Pair) -> bool {
    let shifted_is_empty = old.0.len() == 0;
    let normal_is_empty = old.1.len() == 0;
    let shifted_is_a_letter = old.0.chars().all(|c| c.is_ascii_alphabetic());
    let normal_is_a_letter = old.1.chars().all(|c| c.is_ascii_alphabetic());
    let new_is_a_letter = new.0.chars().all(|c| c.is_ascii_alphabetic());

    (shifted_is_empty && normal_is_empty)
      || !(shifted_is_a_letter || normal_is_a_letter)
      || !new_is_a_letter 
  }

  fn merge_symbols(self: &Self, old: &String, new: &String) -> String {
    if old.len() == 0 {
      new.clone()
    } else {
      old.clone()
    }
  }

  fn random_parents(self: &Self, l1: &Layout, l2: &Layout) -> (DNA, DNA) {
    let d1 = DNA::from(&l1);
    let d2 = DNA::from(&l2);

    if self.random_positive() {
      (d1, d2)
    } else {
      (d2, d1)
    }
  }

  fn random_positive(self: &Self) -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..2) == 0
  }

  fn blank_dna(self: &Self, original: &DNA) -> DNA {
    let mut sequence = Sequence::new();

    for (i, pair) in original.iter().enumerate() {
      let shifted = self.symbol_or_blank((i, 0));
      let normal = self.symbol_or_blank((i, 1));

      sequence.push((shifted, normal));
    }

    DNA { sequence }
  }

  fn symbol_or_blank(self: &Self, position: Position) -> String {
    match self.presie.symbol_at(position) {
      Some(symbol) => symbol.clone(),
      _ => "".to_string()
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use hashbrown::HashSet;

  #[test]
  fn it_sexes_layouts() {
    let splicer = Splicer::new("");

    let qwerty = Layout { template: QWERTY.to_string() };
    let dvorak = Layout { template: DVORAK.to_string() };

    let child = splicer.sex(&qwerty, &dvorak);
  }

  fn s(vec: Vec<(&str,&str)>) -> Vec<(String, String)> {
    vec.iter().map(|(s,n)| (s.to_string(), n.to_string())).collect()
  }

  #[test]
  fn it_merges_chunks_good() {
    let splicer = Splicer::new("");

    let clean = splicer.merge(
      &s(vec![("", ""), ("", ""), ("", ""), ("", "")]),
      &s(vec![("~", "`"), ("!", "1"), ("@", "2"), ("#", "3")])
    );
    assert_eq!(clean, s(vec![("~", "`"), ("!", "1"), ("@", "2"), ("#", "3")]));

    let preserved_symbols = splicer.merge(
      &s(vec![("X", "x"), ("?", ""), ("", "!"), ("", "")]),
      &s(vec![("~", "`"), ("!", "1"), ("@", "2"), ("#", "3")])
    );
    assert_eq!(preserved_symbols, s(vec![("X", "x"), ("?", "1"), ("@", "!"), ("#", "3")]));

    let preserved_letters = splicer.merge(
      &s(vec![("X", "x"), ("?", ""), ("", "!"), ("", "")]),
      &s(vec![("Q", "q"), ("W", "w"), ("E", "e"), ("R", "r")])
    );
    assert_eq!(preserved_letters, s(vec![("X", "x"), ("?", ""), ("", "!"), ("R", "r")]));
  }

  #[test]
  fn it_chunks_dnas_for_days() {
    let splicer = Splicer::new("");

    let qwerty = Layout { template: QWERTY.to_string() };
    let chunks = splicer.chunk(&DNA::from(&qwerty));

    assert_eq!(chunks, vec![
      s(vec![("~", "`"), ("!", "1"), ("@", "2"), ("#", "3"), ("$", "4"), ("%", "5")]), 
      s(vec![("^", "6"), ("&", "7"), ("*", "8"), ("(", "9"), (")", "0"), ("_", "-")]), 
      s(vec![("+", "="), ("Q", "q"), ("W", "w"), ("E", "e"), ("R", "r"), ("T", "t")]), 
      s(vec![("Y", "y"), ("U", "u"), ("I", "i"), ("O", "o"), ("P", "p"), ("{", "[")]), 
      s(vec![("}", "]"), ("|", "\\"), ("A", "a"), ("S", "s"), ("D", "d"), ("F", "f")]), 
      s(vec![("G", "g"), ("H", "h"), ("J", "j"), ("K", "k"), ("L", "l"), (":", ";")]), 
      s(vec![("\"", "\'"), ("Z", "z"), ("X", "x"), ("C", "c"), ("V", "v"), ("B", "b")]), 
      s(vec![("N", "n"), ("M", "m"), ("<", ","), (">", "."), ("?", "/")])
    ]);
  }
  
  #[test]
  fn test_random_positive() {
    let mut set: HashSet<bool> = HashSet::new();
    let splicer = Splicer::new("");

    for _ in 0..2000 {
      set.insert(splicer.random_positive());
    }

    assert_eq!(set.len(), 2);
  }
}