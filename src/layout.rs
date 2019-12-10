#[path = "keys.rs"]
mod keys;
use keys::*;


pub fn parse(layout: String) -> Vec<KEY> {
  let mut stack = Vec::new();

  for symbol in layout.chars() {
    match symbol {
      'a' => stack.push(KEY_A),
      'b' => stack.push(KEY_B),
      'c' => stack.push(KEY_C),
      'd' => stack.push(KEY_D),
      'e' => stack.push(KEY_E),
      'f' => stack.push(KEY_F),
      'g' => stack.push(KEY_G),
      'h' => stack.push(KEY_H),
      'i' => stack.push(KEY_I),
      'j' => stack.push(KEY_J),
      'k' => stack.push(KEY_K),
      'm' => stack.push(KEY_M),
      'n' => stack.push(KEY_N),
      'l' => stack.push(KEY_L),
      'o' => stack.push(KEY_O),
      'p' => stack.push(KEY_P),
      'q' => stack.push(KEY_Q),
      'r' => stack.push(KEY_R),
      's' => stack.push(KEY_S),
      't' => stack.push(KEY_T),
      'u' => stack.push(KEY_U),
      'v' => stack.push(KEY_V),
      'w' => stack.push(KEY_W),
      'x' => stack.push(KEY_X),
      'y' => stack.push(KEY_Y),
      'z' => stack.push(KEY_Z),
      _ => {},
    }
    println!("Looking up: {:}", symbol)
  }

  stack
}

#[cfg(test)]
mod test {
  use super::*;
  use keys::*;

  #[test]
  fn it_works() {
    let layout = "
      s h n t
    ";
    let result = parse(layout.to_string());

    assert_eq!(result, vec![KEY_S, KEY_H, KEY_N, KEY_T])
  }
}