use std::io;
use termion::raw::IntoRawMode;
use termion::event::Key;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders, Paragraph};
use tui::layout::{Layout, Constraint, Direction};

use crate::events::{self,Event};
use crate::generation::Outcome;

pub fn render() -> Result<(), Box<dyn std::error::Error>> {
  let stdout = io::stdout().into_raw_mode()?;
  let backend = TermionBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let mut outcomes: Vec<Outcome> = vec![];
  let mut frames: usize = 0;

  terminal.clear();

  loop {
    frames += 1;

    terminal.draw(|f| {
      let chunks = Layout::default()
          .direction(Direction::Vertical)
          .margin(1)
          .constraints(
              [
                  Constraint::Percentage(70),
                  Constraint::Percentage(30)
              ].as_ref()
          )
          .split(f.size());
      let block1 = Block::default()
            .title("Block")
            .borders(Borders::ALL);
      // f.render_widget(block, chunks[0]);
      let block2 = Block::default()
            .title("Block 2")
            .borders(Borders::ALL);
      f.render_widget(block2, chunks[1]);

      let resutls = outcomes.iter().map(|outcome| {
        format!("{} \t{:?}", outcome.best.name(), outcome.best_summary.score())
      }).collect::<Vec<String>>().join("\n");

      let text = format!(
        "Frame: {}\nGenerations: {}\nResults:\n{}",
        frames, outcomes.len(), resutls
      );

      let par = Paragraph::new(text.clone())
        .block(block1);
      f.render_widget(par, chunks[0]);
    })?;

    match events::inst().next()? {
      Event::Result(outcome) => { // calculation result
        outcomes.push(outcome);
      },
      Event::Input(key) => {  // any key
        if key == Key::Char('q') {
          terminal.clear();
          break; 
        }
      },
      Event::Tick => {} // rebuild the view
    }
  }
  Ok(())
}