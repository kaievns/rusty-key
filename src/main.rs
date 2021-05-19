mod source;
mod config;
mod parser;
mod layout;
mod geometry;
mod keyboard;
mod calculator;
mod summary;
mod layouts;

use crate::keyboard::*;
use crate::calculator::*;
use crate::layouts::{QUERTY, DVORAK, COLEMAK, WORKMAN, THE_1, HALMAK_21};

fn main() -> Result<(), std::io::Error> {
  let data = source::load(String::from("text"))?;
  println!("Loaded text: {:}", data.len());

  let layouts = [
    ("QUERTY", Keyboard::parse(QUERTY)),
    ("DVORAK", Keyboard::parse(DVORAK)),
    ("COLEMAK", Keyboard::parse(COLEMAK)),
    ("WORKMAN", Keyboard::parse(WORKMAN)),
    ("THE-1", Keyboard::parse(THE_1)),
    ("HALMAK 2.1", Keyboard::parse(HALMAK_21))
  ];

  for (name, layout) in layouts.iter() {
    println!("{}: \n{}", name, layout);
    let calculator = Calculator::from(&layout);
    let summary = calculator.run(&data.to_string());
    println!("Score:\neffort: {}\nawkwardness: {}\nrollingness: {}\n", 
      summary.effort, summary.awkwardness, summary.rollingness
    );
  }

  Ok(())
}
