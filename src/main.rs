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
mod model;
mod events;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let evolution = evolution::Evolution::new();
  evolution.start();

  ui::render()
}
