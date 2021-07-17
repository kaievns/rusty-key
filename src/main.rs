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
mod dna;
mod ui;
mod events;

use crate::keyboard::*;
use crate::calculator::*;
use crate::config::*;
use crate::layout::*;

use crate::evolution::*;

fn main() {
  let evolution = Evolution::new();

  ui::render();

  evolution.start();
}


// fn compare_known() {
//   let layouts = [
//     ("QWERTY", &QWERTY ),
//     ("DVORAK", &DVORAK ),
//     ("COLEMAK", &COLEMAK ),
//     ("WORKMAN", &WORKMAN ),
//     ("THE-1", &THE_1 ),
//     ("HALMAK 2.1", &HALMAK_21 )
//   ];

//   for (name, layout) in layouts.iter() {
//     println!("{}: \n{}", name, layout);
//     let keyboard = Keyboard::from(&layout, &CONFIG.geometry);
//     let calculator = Calculator::from(&keyboard);
//     let summary = calculator.run(&CONFIG.data);
//     let fitness = profiler::calculate_fitness(&keyboard);

//     println!("\n{}\nfitness: {}\n", summary, fitness);
//   }
// }