mod source;
mod config;
mod parser;
mod layout;
mod geometry;
mod keyboard;
mod calculator;
mod frequency;
mod summary;

use crate::keyboard::*;
use crate::calculator::*;
use crate::config::{DEFAULT_GEOMETRY};
use crate::layout::{QWERTY, DVORAK, COLEMAK, WORKMAN, THE_1, HALMAK_21};

fn main() -> Result<(), std::io::Error> {
  let data = source::load(String::from("text"))?;
  println!("Loaded text: {:}", data.len());

  let layouts = [
    ("QWERTY", Keyboard::from(QWERTY, DEFAULT_GEOMETRY)),
    ("DVORAK", Keyboard::from(DVORAK, DEFAULT_GEOMETRY)),
    ("COLEMAK", Keyboard::from(COLEMAK, DEFAULT_GEOMETRY)),
    ("WORKMAN", Keyboard::from(WORKMAN, DEFAULT_GEOMETRY)),
    ("THE-1", Keyboard::from(THE_1, DEFAULT_GEOMETRY)),
    ("HALMAK 2.1", Keyboard::from(HALMAK_21, DEFAULT_GEOMETRY))
  ];

  for (name, layout) in layouts.iter() {
    println!("{}: \n{}", name, layout);
    let calculator = Calculator::from(&layout);
    let summary = calculator.run(&data.to_string());
    println!("\n{}\n", summary);
  }

  Ok(())
}
