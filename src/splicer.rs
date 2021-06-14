use crate::dna::*;
use crate::layout::*;
use crate::parser::*;
use crate::preservative::*;

pub struct Splicer {
  presie: Preservative
}

impl Splicer {
  pub fn new(preserve: &'static str) -> Splicer {
    let presie = Preservative::from(preserve);
    Splicer { presie }
  }

  pub fn sex(self: &Self, mom: &Layout, dad: &Layout) -> Layout {
    let mom_dna = DNA::from(&mom);
    let dad_dna = DNA::from(&dad);

    let offspring = self.blank_dna(&mom_dna);

    // magic here

    offspring.to_layout()
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