use std::io;
use termion::raw::IntoRawMode;
use termion::event::Key;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

use crate::events::{Event, Events};
use crate::stats::*;

pub fn render(stats: &Stats) -> Result<(), Box<dyn std::error::Error>> {
  let stdout = io::stdout().into_raw_mode()?;
  let backend = TermionBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let events = Events::new();
  terminal.clear();

  loop {
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
      let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
      f.render_widget(block, chunks[0]);
      let block = Block::default()
            .title("Block 2")
            .borders(Borders::ALL);
      f.render_widget(block, chunks[1]);
    })?;

    match events.next()? {
      Event::Input(_) => {  // any key 
        terminal.clear();
        break; 
      },
      Event::Tick => {} // rebuild the view
    }
  }
  Ok(())
}