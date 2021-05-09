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
  println!("QUERTY: \n{}", querty);
  

  let calculator_querty = Calculator::from(&querty);
  println!("Score:\n{}\n", calculator_querty.run(&data.to_string()));

  let halmak_21 = Keyboard::halmak_21();
  println!("HALMAK 2.1: \n{}", halmak_21);
  let calculator_halmak_21 = Calculator::from(&halmak_21);
  println!("Score:\n{}", calculator_halmak_21.run(&data.to_string()));

  Ok(())
}
