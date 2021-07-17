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
use crate::config::*;
use crate::layout::*;

use crate::summary::*;

fn main() {
  let layouts = [
    ("QWERTY", &QWERTY.clone()),
    ("DVORAK", &DVORAK.clone()),
    ("COLEMAK", &COLEMAK.clone()),
    ("WORKMAN", &WORKMAN.clone()),
    ("THE-1", &THE_1.clone()),
    ("HALMAK 2.1", &HALMAK_21.clone())
  ];

  for (name, layout) in layouts.iter() {
    println!("{}: \n{}", name, layout);
    let keyboard = Keyboard::from(&layout, &CONFIG.geometry);
    let summary = Summary::calculate(&keyboard);

    println!("\n{}\n", summary);
  }

  // let evolution = Evolution::new();

  // ui::render();

  // evolution.start();
}
