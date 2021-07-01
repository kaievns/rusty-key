mod source;
mod config;
mod parser;
mod layout;
mod geometry;
mod keyboard;
mod calculator;
mod preservative;
mod population;
mod generation;
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
    ("QWERTY", Layout { template: QWERTY.to_string() } ),
    ("DVORAK", Layout { template: DVORAK.to_string() } ),
    ("COLEMAK", Layout { template: COLEMAK.to_string() } ),
    ("WORKMAN", Layout { template: WORKMAN.to_string() } ),
    ("THE-1", Layout { template: THE_1.to_string() } ),
    ("HALMAK 2.1", Layout { template: HALMAK_21.to_string() } )
  ];

  for (name, layout) in layouts.iter() {
    println!("{}: \n{}", name, layout);
    let keyboard = Keyboard::from(&layout, &DEFAULT_GEOMETRY);
    let calculator = Calculator::from(&keyboard);
    let summary = calculator.run(&data.to_string());
    println!("\n{}\n", summary);
  }

  Ok(())
}
