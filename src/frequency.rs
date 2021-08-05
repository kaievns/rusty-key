use hashbrown::HashMap;
use once_cell::sync::Lazy;

use crate::config::CONFIG;

type UsageFrequencies = HashMap<String, usize>;
pub type SymbolFrequencies = Vec<(char, usize)>;

pub const CURRENT_FREQUENCIES: Lazy<SymbolFrequencies> = Lazy::new(|| {
  calculate_frequencies(&CONFIG.data).into_iter()
    .map(|(symbol, count)| (symbol.chars().next().unwrap(), count))
    .collect::<SymbolFrequencies>()
});

pub fn calculate_frequencies(text: &String) -> UsageFrequencies {
  let mut usage = UsageFrequencies::new();

  for symbol in text.chars() {
    *usage.entry(symbol.to_string()).or_insert(0) += 1;
  }

  usage
}

#[cfg(test)]
mod test {
  use super::*;

  macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::hashbrown::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
  );

  #[test]
  fn calculates_usage_frequencies() {
    let text = "Hello World!\nBlargh!";

    assert_eq!(calculate_frequencies(&text.to_string()), map! {
      "H".to_string() => 1, 
      "d".to_string() => 1, 
      "e".to_string() => 1, 
      "o".to_string() => 2, 
      "r".to_string() => 2, 
      "\n".to_string()=> 1, 
      "a".to_string() => 1, 
      "!".to_string() => 2, 
      "g".to_string() => 1, 
      "l".to_string() => 4, 
      " ".to_string() => 1, 
      "B".to_string() => 1, 
      "h".to_string() => 1, 
      "W".to_string() => 1
    });
  }

}