use rand::Rng;

use hashbrown::HashSet;

use crate::layout::*;
use crate::parser::*;

pub struct Mutator {
  preserved_positions: HashSet<Position>,
  cache: HashSet<String>
}

type Pair = (String, String);
type DNA = Vec<Pair>;

fn parse_positions(template: &'static str) -> HashSet<Position> {
  let tmp_layout = Layout { template: template.to_string() };
  let mut preserved_positions: HashSet<Position> = HashSet::new();

  for (position, entry) in tmp_layout.entries().iter().enumerate() {
    let shifted_is_okay = entry.shifted.chars().all(|c|
      c.is_ascii_punctuation() || c.is_ascii_alphanumeric()
    );
    let normal_is_okay = entry.normal.chars().all(|c|
      c.is_ascii_punctuation() || c.is_ascii_alphanumeric()
    );

    if shifted_is_okay { preserved_positions.insert((position, 0)); }
    if normal_is_okay { preserved_positions.insert((position, 1)); }
  }

  preserved_positions
}

impl Mutator {
  pub fn new(preserve: &'static str) -> Mutator {
    let preserved_positions = parse_positions(preserve);
    Mutator { preserved_positions, cache: HashSet::new() }
  }

  pub fn mutate_keys(self: &Self, layout: &Layout) -> Layout {
    let dna = self.to_dna(layout);
    let new_dna = self.swap_random_keys(&dna);

    self.from_dna(&new_dna)
  }

  pub fn mutate_symbols(self: &Self, layout: &Layout) -> Layout {
    let dna = self.to_dna(layout);
    let new_dna = self.swap_random_symbols(&dna);

    self.from_dna(&new_dna)
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

  fn swap_random_keys(self: &Self, original: &DNA) -> DNA {
    let (first_pos, second_pos) = self.two_random_key_positions(&original);

    self.swap_keys(original, first_pos, second_pos)
  }

  fn swap_random_symbols(self: &Self, original: &DNA) -> DNA {
    let (first_pos, second_pos) = self.two_random_symbol_positions(&original);

    self.swap_symbols(&original, first_pos, second_pos)
  }

  // NOTE: usafe, will swap regardless restrictions
  fn swap_keys(self: &Self, original: &DNA, pos1: usize, pos2: usize) -> DNA {
    let shifts_swapped = self.swap_symbols(&original, (pos1, 0), (pos2, 0));
    
    self.swap_symbols(&shifts_swapped, (pos1, 1), (pos2, 1))
  }

  // NOTE: usafe, will swap regardless restrictions
  fn swap_symbols(self: &Self, original: &DNA, pos1: Position, pos2: Position) -> DNA {
    let mut new_dna = original.clone();

    let entry1 = new_dna.get(pos1.0).unwrap();
    let entry2 = new_dna.get(pos2.0).unwrap();

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

    if let Some(entry) = new_dna.get_mut(pos1.0) {
      *entry = new_entry1;
    }

    if let Some(entry) = new_dna.get_mut(pos2.0) {
      *entry = new_entry2;
    }
    
    new_dna
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

      let shifted_is_safe = self.is_safe_position((position, 0));
      let normal_is_safe = self.is_safe_position((position, 1));

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

      if self.is_safe_position(position) { break; }
    }

    position
  }

  fn is_safe_position(self: &Self, position: Position) -> bool {
    !self.preserved_positions.contains(&position)
  }

  fn random_number(self: &Self, size: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..size)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  macro_rules! set {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_set = HashSet::new();
            $(
                temp_set.insert($x);
            )*
            temp_set
        }
    };
  }

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
  fn parsing_preserved_positions() {
    let list = parse_positions("
      ∙ ! ∙ * ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙
      ∙ 1 ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙
        Q ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙
        q ∙ r ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙
        ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ 
        ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ 
          ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ ∙ 
          ∙ ∙ ∙ ∙ ∙ ∙ ∙ , . ∙ 
    ");
    assert_eq!(list, set! [
      (1,0), (1,1),
      (3,0),
      (13, 0), (13, 1),
      (15, 1),
      (44, 1),
      (45, 1)
    ]);
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

    println!("{:?}", dna[1]);
    println!("{:?}", dna[3]);
    println!("{:?}", dna[13]);
    println!("{:?}", dna[15]);
    println!("{:?}", dna[44]);

    assert_eq!(("!".to_string(), "1".to_string()), dna[1]);
    assert_eq!(("Q".to_string(), "q".to_string()), dna[13]);
    assert_eq!("#".to_string(), dna[3].0);
    assert_eq!(",".to_string(), dna[44].1);

    assert_ne!("W".to_string(), dna[14].0);
    assert_ne!("w".to_string(), dna[14].1);
    assert_ne!("3".to_string(), dna[3].1);
    assert_ne!("<".to_string(), dna[44].0);
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
  fn test_swapping_keys() {
    let mutator = Mutator::new("");
    let original = qwerty_dna();
    let new_dna = mutator.swap_keys(&original, 2, 5);

    assert_eq!(original, qwerty_dna()); // should not change
    assert_eq!(new_dna, [
      &original[0..2],
      &[
        ("%".to_string(), "5".to_string()),
        ("#".to_string(), "3".to_string()), 
        ("$".to_string(), "4".to_string()), 
        ("@".to_string(), "2".to_string()) 
      ],
      &original[6..]
    ].concat().to_vec())
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
      let entry1 = sequence.get(pos1.0).unwrap();
      let entry2 = sequence.get(pos2.0).unwrap();

      assert_ne!(pos1, pos2);

      assert_eq!(true, entry1.0.chars().all(|c| !c.is_ascii_alphabetic()), "{:?}", entry1.0);
      assert_eq!(true, entry1.1.chars().all(|c| !c.is_ascii_alphabetic()), "{:?}", entry1.1);
      assert_eq!(true, entry2.0.chars().all(|c| !c.is_ascii_alphabetic()), "{:?}", entry2.0);
      assert_eq!(true, entry2.1.chars().all(|c| !c.is_ascii_alphabetic()), "{:?}", entry2.1);
    }
  }

  #[test]
  fn swapping_symbols() {
    let mutator = Mutator::new("");
    let original = qwerty_dna();
    let new_dna1 = mutator.swap_symbols(&original, (2,0), (3,0));
    let new_dna2 = mutator.swap_symbols(&original, (2,0), (3,1));
    let new_dna3 = mutator.swap_symbols(&original, (2,1), (3,0));
    let new_dna4 = mutator.swap_symbols(&original, (2,1), (3,1));

    assert_eq!(original, qwerty_dna()); // should not change

    assert_eq!(new_dna1, [
      &original[0..2],
      &[
        ("#".to_string(), "2".to_string()), 
        ("@".to_string(), "3".to_string()),
      ],
      &original[4..]
    ].concat().to_vec());

    assert_eq!(new_dna2, [
      &original[0..2],
      &[
        ("3".to_string(), "2".to_string()), 
        ("#".to_string(), "@".to_string()), 
      ],
      &original[4..]
    ].concat().to_vec());

    assert_eq!(new_dna3, [
      &original[0..2],
      &[
        ("@".to_string(), "#".to_string()), 
        ("2".to_string(), "3".to_string()), 
      ],
      &original[4..]
    ].concat().to_vec());

    assert_eq!(new_dna4, [
      &original[0..2],
      &[
        ("@".to_string(), "3".to_string()), 
        ("#".to_string(), "2".to_string()), 
      ],
      &original[4..]
    ].concat().to_vec());
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