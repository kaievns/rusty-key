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
mod selection;
mod evolution;
mod frequency;
mod profiler;
mod summary;
mod mutator;
mod stats;
mod dna;

use crate::keyboard::*;
use crate::calculator::*;
use crate::config::*;
use crate::layout::*;

fn main() -> Result<(), std::io::Error> {
  println!("Loaded text: {:}", CONFIG.data.len());

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
    let keyboard = Keyboard::from(&layout, &CONFIG.geometry);
    let calculator = Calculator::from(&keyboard);
    let summary = calculator.run(&CONFIG.data);
    let fitness = profiler::calculate_fitness(&keyboard);

    println!("\n{}\nfitness: {}\n", summary, fitness);
  }

  Ok(())
}
