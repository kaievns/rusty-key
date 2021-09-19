use crate::layout::*;
use crate::parser::*;

pub type Pair = (String, String);
pub type Sequence = Vec<Pair>;

#[derive(Debug,PartialEq,Eq)]
pub struct DNA {
  pub sequence: Sequence
}

impl DNA {
  pub fn from(layout: &Layout) -> DNA {
    let mut sequence = Sequence::new();

    for entry in layout.entries() {
      sequence.push((entry.shifted.to_owned(), entry.normal.to_owned()));
    }

    DNA { sequence }
  }

  pub fn to_layout(self: &Self) -> Layout {
    let mut shifts = "".to_string();
    let mut normals = "".to_string();
    
    for (i, (shifted, normal)) in self.sequence.iter().enumerate() {
      shifts = format!("{} {}", shifts, shifted);
      normals = format!("{} {}", normals, normal);
  
      match i {
        12 | 25 | 36 => {
          let spacer = if i == 36 { "   " } else { " " };

          shifts = format!("{}\n{}", shifts.trim(), spacer);
          normals = format!("{}\n{}", normals.trim(), spacer);
        },
        _ => {}
      }
    }

    let template = shifts.lines().zip(normals.lines())
      .map(|(s,n)| [s,n].join("\n"))
      .collect::<Vec<String>>()
      .join("\n");

    Layout { template: template }
  }

  pub fn swap_keys(self: &Self, pos1: usize, pos2: usize) -> DNA {
    let shifts_swapped = self.swap_symbols((pos1, 0), (pos2, 0));
    shifts_swapped.swap_symbols((pos1, 1), (pos2, 1))
  }

  pub fn swap_symbols(self: &Self, pos1: Position, pos2: Position) -> DNA {
    let mut sequence = self.sequence.clone();

    let entry1 = sequence.get(pos1.0).unwrap();
    let entry2 = sequence.get(pos2.0).unwrap();

    let new_entry1 = if pos1.1 == 0 {
      if pos2.1 == 0 { (entry2.0.clone(), entry1.1.clone()) } else { (entry2.1.clone(), entry1.1.clone()) }
    } else {
      if pos2.1 == 0 { (entry1.0.clone(), entry2.0.clone()) } else { (entry1.0.clone(), entry2.1.clone()) }
    };
    let new_entry2 = if pos2.1 == 0 {
      if pos1.1 == 0 { (entry1.0.clone(), entry2.1.clone()) } else { (entry1.1.clone(), entry2.1.clone()) }
    } else {
      if pos1.1 == 0 { (entry2.0.clone(), entry1.0.clone()) } else { (entry2.0.clone(), entry1.1.clone()) }
    };

    if let Some(entry) = sequence.get_mut(pos1.0) {
      *entry = new_entry1;
    }

    if let Some(entry) = sequence.get_mut(pos2.0) {
      *entry = new_entry2;
    }
    
    DNA { sequence }
  }

  pub fn len(self: &Self) -> usize {
    self.sequence.len()
  }

  pub fn iter(self: &Self) -> std::slice::Iter<Pair> {
    self.sequence.iter()
  }

  #[allow(dead_code)]
  pub fn pair_at(self: &Self, i: usize) -> Pair {
    self.sequence[i].clone()
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_from_layout() {
    let dna = DNA::from(&QWERTY.clone());
    
    assert_eq!(dna.sequence, qwerty_sequence());
  }

  #[test]
  fn test_to_layout() {
    let dna = DNA { sequence: qwerty_sequence() };

    assert_eq!(dna.to_layout(), QWERTY.clone());
  }

  #[test]
  fn test_swapping_keys() {
    let original = DNA { sequence: qwerty_sequence() };
    let new_dna = original.swap_keys(2, 5);

    assert_eq!(original.sequence, qwerty_sequence()); // should not change
    assert_eq!(new_dna.sequence, [
      &original.sequence[0..2],
      &[
        ("%".to_string(), "5".to_string()),
        ("#".to_string(), "3".to_string()), 
        ("$".to_string(), "4".to_string()), 
        ("@".to_string(), "2".to_string()) 
      ],
      &original.sequence[6..]
    ].concat().to_vec())
  }

  #[test]
  fn test_swapping_symbols() {
    let original = DNA { sequence: qwerty_sequence() };
    let new_dna1 = original.swap_symbols((2,0), (3,0));
    let new_dna2 = original.swap_symbols((2,0), (3,1));
    let new_dna3 = original.swap_symbols((2,1), (3,0));
    let new_dna4 = original.swap_symbols((2,1), (3,1));

    assert_eq!(original.sequence, qwerty_sequence()); // should not change

    assert_eq!(new_dna1.sequence, [
      &original.sequence[0..2],
      &[
        ("#".to_string(), "2".to_string()), 
        ("@".to_string(), "3".to_string()),
      ],
      &original.sequence[4..]
    ].concat().to_vec());

    assert_eq!(new_dna2.sequence, [
      &original.sequence[0..2],
      &[
        ("3".to_string(), "2".to_string()), 
        ("#".to_string(), "@".to_string()), 
      ],
      &original.sequence[4..]
    ].concat().to_vec());

    assert_eq!(new_dna3.sequence, [
      &original.sequence[0..2],
      &[
        ("@".to_string(), "#".to_string()), 
        ("2".to_string(), "3".to_string()), 
      ],
      &original.sequence[4..]
    ].concat().to_vec());

    assert_eq!(new_dna4.sequence, [
      &original.sequence[0..2],
      &[
        ("@".to_string(), "3".to_string()), 
        ("#".to_string(), "2".to_string()), 
      ],
      &original.sequence[4..]
    ].concat().to_vec());
  }

  fn qwerty_sequence() -> Sequence {
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