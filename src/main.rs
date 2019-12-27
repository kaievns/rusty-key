mod source;
mod config;
mod layout;
mod keyboard;
mod calculator;

use crate::keyboard::*;
use crate::calculator::*;

fn main() -> Result<(), std::io::Error> {
  let data = source::load(String::from("text"))?;
  println!("Loaded text: {:}", data.len());

  let querty = Keyboard::querty();
  println!("QUERTY: {:?}", querty);

  let calculator = Calculator::from(&querty.layout);
  println!("Score: {:?}", calculator.summary());

  Ok(())
}
