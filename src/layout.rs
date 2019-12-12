#[path = "keys.rs"]
mod keys;
use keys::*;

pub type Layout = Vec<KEY>;

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

      '1' => stack.push(KEY_1),
      '2' => stack.push(KEY_2),
      '3' => stack.push(KEY_3),
      '4' => stack.push(KEY_4),
      '5' => stack.push(KEY_5),
      '6' => stack.push(KEY_6),
      '7' => stack.push(KEY_7),
      '8' => stack.push(KEY_8),
      '9' => stack.push(KEY_9),
      '0' => stack.push(KEY_0),

      '`' => stack.push(KEY_TILDA), 
      '-' => stack.push(KEY_DASH), 
      '+' => stack.push(KEY_PLUS), 
      '[' => stack.push(KEY_SQ_O), 
      ']' => stack.push(KEY_SQ_C), 
      '\\' => stack.push(KEY_COLON),
      ';' => stack.push(KEY_SEMI), 
      '\'' => stack.push(KEY_QUOTE),
      ',' => stack.push(KEY_COMA), 
      '.' => stack.push(KEY_DOT), 
      '/' => stack.push(KEY_SLASH),

      _ => {},
    }
  }

  stack
}

pub fn print(layout: Layout) -> String {
  let mut string = "".to_string();

  for (i, key) in layout.iter().enumerate() {
    string = format!("{} {}", string, key.0);

    match (i) {
      11 => string = format!("{}\n  ", string),
      24 => string = format!("{}\n  ", string),
      35 => string = format!("{}\n   ", string),
      _ => {}
    }
    // println!("{} {:?}", i, key)
  }

  string
}

#[cfg(test)]
mod test {
  use super::*;
  use keys::*;

  #[test]
  fn it_parses() {
    let layout = "
      s h n t
    ";
    let result = parse(layout.to_string());

    assert_eq!(result, vec![KEY_S, KEY_H, KEY_N, KEY_T])
  }

  #[test]
  fn it_prints() {
    let qwerty = "
    ` 1 2 3 4 5 6 7 8 9 0 - =
      q w e r t y u i o p [ ] \\
      a s d f g h j k l ; '
        z x c v b n m , . /
    ".to_string();


    let layout = parse(qwerty);
    let result = print(layout);

    println!("{}", result);

    assert_eq!(result, " ` 1 2 3 4 5 6 7 8 9 0 -\n   q w e r t y u i o p [ ] \\\n   a s d f g h j k l ; \'\n    z x c v b n m , . /")
  }
}