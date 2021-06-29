mod source;
mod config;
mod parser;
mod layout;
mod geometry;
mod keyboard;
mod calculator;
mod preservative;
mod population;
mod frequency;
mod summary;
mod mutator;
mod dna;

use crate::keyboard::*;
use crate::calculator::*;
use crate::config::{DEFAULT_GEOMETRY};
use crate::layout::*;

fn main() -> Result<(), std::io::Error> {
  let data = source::load(String::from("text"))?;
  println!("Loaded text: {:}", data.len());

  let layouts = [
    ("QWERTY", Keyboard::from(Layout { template: QWERTY.to_string() }, DEFAULT_GEOMETRY)),
    ("DVORAK", Keyboard::from(Layout { template: DVORAK.to_string() }, DEFAULT_GEOMETRY)),
    ("COLEMAK", Keyboard::from(Layout { template: COLEMAK.to_string() }, DEFAULT_GEOMETRY)),
    ("WORKMAN", Keyboard::from(Layout { template: WORKMAN.to_string() }, DEFAULT_GEOMETRY)),
    ("THE-1", Keyboard::from(Layout { template: THE_1.to_string() }, DEFAULT_GEOMETRY)),
    ("HALMAK 2.1", Keyboard::from(Layout { template: HALMAK_21.to_string() }, DEFAULT_GEOMETRY))
  ];

  for (name, layout) in layouts.iter() {
    println!("{}: \n{}", name, layout);
    let calculator = Calculator::from(&layout);
    let summary = calculator.run(&data.to_string());
    println!("\n{}\n", summary);
  }

  Ok(())
}
