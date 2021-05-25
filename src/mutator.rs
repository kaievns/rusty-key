use hashbrown::HashSet;

use crate::layout::*;
use crate::parser::*;

pub struct Mutator {
  preserve: &'static str,
  cache: HashSet<DNA>
}

type DNA = Vec<(String, String)>;

impl Mutator {
  pub fn new(preserve: &'static str) -> Mutator {
    Mutator { preserve, cache: HashSet::new() }
  }

  pub fn mutate(self: &Self, layout: &Layout) -> Layout {
    let dna = self.to_dna(layout);

    self.from_dna(&dna)
  }

  fn to_dna(self: &Self, layout: &Layout) -> DNA {
    let mut dna = DNA::new();

    for entry in layout.entries() {
      dna.push((entry.shifted.to_owned(), entry.normal.to_owned()));
    }

    dna
  }

  fn from_dna(self: &Self, dna: &DNA) -> Layout {
    let mut shifts = " ".to_string();
    let mut normals = " ".to_string();
    
    for (i, (shifted, normal)) in dna.iter().enumerate() {
      shifts = format!("{} {}", shifts, shifted);
      normals = format!("{} {}", normals, normal);
  
      match i {
        12 | 25 | 36 => {
          let spacer = if i == 36 { "     " } else { "   " };

          shifts = format!("{}\n{}", shifts, spacer);
          normals = format!("{}\n{}", normals, spacer);
        },
        _ => {}
      }
    }

    let template = shifts.lines().zip(normals.lines())
      .map(|(s,n)| [s,n].join("\n"))
      .collect::<Vec<String>>()
      .join("\n");

    Layout { template: format!("\n{}\n", template) }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_mutates() {
    let layout = Layout { template: QWERTY.to_string() };
    let mutator = Mutator::new("");
    let new_layout = mutator.mutate(&layout);

    assert_eq!(new_layout.template, layout.template);
  }

  #[test]
  fn to_dna() {
    let layout = Layout { template: QWERTY.to_string() };
    let mutator = Mutator::new("");
    
    assert_eq!(mutator.to_dna(&layout), qwerty_dna());
  }

  #[test]
  fn from_dna() {
    let mutator = Mutator::new("");
    let layout = Layout { template: QWERTY.to_string() };

    assert_eq!(mutator.from_dna(&qwerty_dna()), layout);
  }

  fn qwerty_dna() -> DNA {
    vec![
      ("~".to_string(), "`".to_string()), 
      ("!".to_string(), "1".to_string()), 
      ("@".to_string(), "2".to_string()), 
      ("#".to_string(), "3".to_string()), 
      ("$".to_string(), "4".to_string()), 
      ("%".to_string(), "5".to_string()), 
      ("^".to_string(), "6".to_string()), 
      ("&".to_string(), "7".to_string()), 
      ("*".to_string(), "8".to_string()), 
      ("(".to_string(), "9".to_string()), 
      (")".to_string(), "0".to_string()), 
      ("_".to_string(), "-".to_string()), 
      ("+".to_string(), "=".to_string()), 
      ("Q".to_string(), "q".to_string()), 
      ("W".to_string(), "w".to_string()), 
      ("E".to_string(), "e".to_string()), 
      ("R".to_string(), "r".to_string()), 
      ("T".to_string(), "t".to_string()), 
      ("Y".to_string(), "y".to_string()), 
      ("U".to_string(), "u".to_string()), 
      ("I".to_string(), "i".to_string()), 
      ("O".to_string(), "o".to_string()), 
      ("P".to_string(), "p".to_string()), 
      ("{".to_string(), "[".to_string()), 
      ("}".to_string(), "]".to_string()), 
      ("|".to_string(), "\\".to_string()), 
      ("A".to_string(), "a".to_string()), 
      ("S".to_string(), "s".to_string()), 
      ("D".to_string(), "d".to_string()), 
      ("F".to_string(), "f".to_string()), 
      ("G".to_string(), "g".to_string()), 
      ("H".to_string(), "h".to_string()), 
      ("J".to_string(), "j".to_string()), 
      ("K".to_string(), "k".to_string()), 
      ("L".to_string(), "l".to_string()), 
      (":".to_string(), ";".to_string()), 
      ("\"".to_string(), "\'".to_string()), 
      ("Z".to_string(), "z".to_string()), 
      ("X".to_string(), "x".to_string()), 
      ("C".to_string(), "c".to_string()), 
      ("V".to_string(), "v".to_string()), 
      ("B".to_string(), "b".to_string()), 
      ("N".to_string(), "n".to_string()), 
      ("M".to_string(), "m".to_string()), 
      ("<".to_string(), ",".to_string()), 
      (">".to_string(), ".".to_string()), 
      ("?".to_string(), "/".to_string())
    ]
  }
}