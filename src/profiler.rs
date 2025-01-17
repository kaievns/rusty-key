use once_cell::sync::OnceCell;

use crate::frequency::*;
use crate::keyboard::*;

pub fn calculate_fitness(keyboard: &Keyboard) -> f64 {
  let symbols = symbols_by_frequency();
  let efforts = symbols_by_effort(&keyboard);

  let max_score = total_possible_score(symbols);
  let mut score = 0;

  for (i, symbol) in symbols.iter().enumerate() {
    let band = symbols_in_band(&efforts, i);
    let fits = band.iter().any(|s| s == symbol);

    if fits { score += score_for(symbol); }
  }

  3.0 * (score as f64) / (max_score as f64)
}

fn total_possible_score(symbols: &Vec<String>) -> usize {
  static TOTAL_SCORE: OnceCell<usize> = OnceCell::new();
  *TOTAL_SCORE.get_or_init(|| {
    symbols.iter()
      .map(|symbol| score_for(symbol))
      .sum()
  })
}

fn score_for(symbol: &String) -> usize {
  match symbol.chars().nth(0).unwrap() {
    'a'..='z' => 4,
    '0'..='9' => 2,
    _ => 1
  }
}

// returns a list of symbols ordered by usage frequency
fn symbols_by_frequency() -> &'static Vec<String> {
  static SYMBOLS: OnceCell<Vec<String>> = OnceCell::new();
  SYMBOLS.get_or_init(|| {
    let mut sorted = CURRENT_FREQUENCIES.to_vec();
    sorted.sort_by(|a,b| {
      if a.1 == b.1 { a.0.cmp(&b.0) }
      else { b.1.cmp(&a.1) }
    });

    sorted.iter()
      .filter(|(symbol,_)| *symbol != ' ')
      .filter(|(symbol,_)| !symbol.is_ascii_uppercase())
      .map(|(symbol,_)| symbol.to_string())
      .collect()
  })
}

// returns a list of symbol<>effort pairs that represent the keyboard
type SymbolEfforts = Vec<(String, usize)>;
fn symbols_by_effort(keyboard: &Keyboard) -> SymbolEfforts {
  let mut sorted: SymbolEfforts = keyboard.key_map.iter()
    .filter(|(symbol, _)| **symbol != ' ')
    .filter(|(symbol, _)| !symbol.is_ascii_uppercase())
    .map(|(symbol, key)| (symbol.to_string(), key.effort))
    .collect();

  sorted.sort_by(|a,b| {
    if a.1 == b.1 { a.0.cmp(&b.0) }
    else { a.1.cmp(&b.1) }
  });

  sorted
}

// returns the list of symbols with the same effort around given index
fn symbols_in_band(symbols_with_efforts: &SymbolEfforts, index: usize) -> Vec<String> {
  match symbols_with_efforts.get(index) {
    None => vec![],
    Some(pair) => symbols_with_efforts.iter()
      .filter(|(_, effort)| *effort == pair.1)
      .map(|(symbol,_)| symbol.to_string())
      .collect()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::layout::*;
  use crate::geometry::*;

  #[test]
  fn test_total_possible_score() {
    let symbols = symbols_by_frequency();
    
    assert_eq!(total_possible_score(&symbols), 106);
  }

  #[test]
  fn test_score_for() {
    assert_eq!(score_for(&"a".to_string()), 4);
    assert_eq!(score_for(&"z".to_string()), 4);
    assert_eq!(score_for(&"0".to_string()), 2);
    assert_eq!(score_for(&"9".to_string()), 2);
    assert_eq!(score_for(&"<".to_string()), 1);
    assert_eq!(score_for(&">".to_string()), 1);
  }

  #[test]
  fn test_symbols_by_frequency() {
    let most_frequent_symbols = symbols_by_frequency();
    let top10 = &most_frequent_symbols[0..10];
    assert_eq!(top10, vec!["e", "a", "o", "i", "s", "n", "t", "r", "l", "p"]);
  }

  #[test]
  fn test_symbols_by_effort() {
    let layout = QWERTY.clone();
    let keyboard = Keyboard::from(&layout, &US_PC_KEYBOARD);
    let symbols_with_efforts = symbols_by_effort(&keyboard);
    let top10 = &symbols_with_efforts[0..10];

    assert_eq!(top10, vec![
      ("d".to_string(), 0), 
      ("f".to_string(), 0), 
      ("j".to_string(), 0), 
      ("k".to_string(), 0), 
      ("l".to_string(), 0), 
      ("s".to_string(), 0), 
      (";".to_string(), 1), 
      ("a".to_string(), 1), 
      ("e".to_string(), 1), 
      ("i".to_string(), 1)
    ]);
  }

  #[test]
  fn test_symbols_in_band() {
    let layout = QWERTY.clone();
    let keyboard = Keyboard::from(&layout, &US_PC_KEYBOARD);
    let symbols_with_efforts = symbols_by_effort(&keyboard);

    assert_eq!(
      symbols_in_band(&symbols_with_efforts, 1), 
      vec!["d", "f", "j", "k", "l", "s"]
    );
    assert_eq!(
      symbols_in_band(&symbols_with_efforts, 8), 
      vec![";", "a", "e", "i", "o"]
    );

    assert_eq!(symbols_in_band(&symbols_with_efforts, 98), vec![] as Vec<String>);
  }

  fn get_fitness_for(layout: &Layout) -> f64 {
    let keyboard = Keyboard::from(layout, &US_PC_KEYBOARD);

    calculate_fitness(&keyboard)
  }
  
  #[test]
  fn test_selection() {   
    assert_eq!(get_fitness_for(&QWERTY), 0.42452830188679247);
    assert_eq!(get_fitness_for(&DVORAK), 0.5660377358490566);
    assert_eq!(get_fitness_for(&COLEMAK), 0.8207547169811321);
    assert_eq!(get_fitness_for(&WORKMAN), 0.7075471698113207);
    assert_eq!(get_fitness_for(&HALMAK_21), 0.7924528301886793);
  }
}