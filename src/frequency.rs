use hashbrown::HashMap;

type UsageFrequencies = HashMap<String, usize>;

#[allow(dead_code)]
pub fn calculate_frequencies(text: &String) -> UsageFrequencies {
  let mut usage = UsageFrequencies::new();

  for symbol in text.chars() {
    *usage.entry(symbol.to_string()).or_insert(0) += 1;
  }

  usage
}

#[allow(dead_code)]
pub const ENGLISH_TEXT_FREQUENCIES: [(char, usize); 89] = [
  ('R', 1303), 
  ('>', 14), 
  ('_', 10), 
  ('q', 1070), 
  ('h', 62414), 
  ('.', 15275), 
  ('C', 2190), 
  ('t', 107555), 
  ('Q', 73), 
  ('G', 1287), 
  ('7', 234), 
  ('*', 33), 
  ('w', 21701), 
  ('J', 1080), 
  ('j', 1449), 
  ('4', 243), 
  ('%', 40), 
  ('|', 1), 
  ('M', 2639), 
  ('a', 98926), 
  ('U', 662), 
  ('E', 1014), 
  ('1', 1183), 
  ('"', 7723), 
  (',', 18664), 
  ('/', 280), 
  ('P', 1506), 
  ('r', 73170), 
  ('A', 3904), 
  ('!', 182), 
  ('[', 44), 
  ('D', 1440), 
  ('m', 29177), 
  ('I', 5217), 
  ('F', 1061), 
  ('\'', 5867), 
  ('W', 2108), 
  ('?', 577), 
  ('g', 24272), 
  ('&', 29), 
  ('T', 4343), 
  ('i', 85748), 
  ('0', 1413), 
  ('H', 2297), 
  ('v', 12668), 
  ('c', 35199), 
  ('\n', 7913), 
  ('N', 1198), 
  ('e', 149308), 
  ('y', 22322), 
  ('(', 541), 
  (']', 44), 
  (' ', 255247), 
  ('3', 315), 
  ('z', 1225), 
  ('o', 89116), 
  ('Y', 579), 
  ('O', 1397), 
  (')', 551), 
  ('9', 609), 
  ('b', 17084),
  ('V', 421), 
  ('S', 3506), 
  ('8', 213), 
  ('L', 1078), 
  (':', 963),
  ('X', 39), 
  ('B', 2543), 
  ('p', 23772), 
  ('l', 48996), 
  ('-', 4748), 
  ('@', 2), 
  ('‚', 1), 
  ('6', 304), 
  ('k', 9355), 
  ('$', 63), 
  ('5', 328), 
  ('\t', 11), 
  ('+', 1), 
  ('s', 78752), 
  ('n', 84857), 
  ('#', 87), 
  ('Z', 180), 
  (';', 436), 
  ('2', 899), 
  ('f', 23966), 
  ('x', 2210), 
  ('d', 47776), 
  ('K', 604)
];

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