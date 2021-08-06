use std::io;
use termion::raw::IntoRawMode;
use termion::event::Key;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::text::{Span};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Cell, Row, Table};
use tui::layout::{Layout, Constraint, Direction};

use crate::events::{self,Event};
use crate::model::ViewModel;

pub fn render() -> Result<(), Box<dyn std::error::Error>> {
  let stdout = io::stdout().into_raw_mode()?;
  let backend = TermionBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let mut model = ViewModel::new();

  terminal.clear();

  loop {
    terminal.draw(|f| {
      
      let main_chunks = Layout::default()
          .direction(Direction::Horizontal)
          .margin(1)
          .constraints(
            [
              Constraint::Percentage(70),
              Constraint::Percentage(30)
            ].as_ref()
          )
          .split(f.size());

      let chunks = Layout::default()
          .direction(Direction::Vertical)
          // .margin(1)
          .constraints(
              [
                  Constraint::Percentage(70),
                  Constraint::Percentage(30)
              ].as_ref()
          )
          .split(main_chunks[0]);

      let table_block = Block::default()
            .title("Latest results")
            .borders(Borders::ALL);
      // f.render_widget(table_block, chunks[0]);

      let header_cells = ["#", "Layout name", "Score"].iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
      let header = Row::new(header_cells);
                // .style(normal_style)
                // .height(1)
                // .bottom_margin(1);
      let top_list = model.top_list();
      let rows = top_list.iter().map(|line| {
        let cells = line.iter().map(|c| Cell::from(Span::raw(c)));
        Row::new(cells) //.bottom_margin(1)
      });
      let table = Table::new(rows)
        .header(header)
        .block(table_block)
        .widths(&[
          Constraint::Length(3),
          // Constraint::Min(10),
          Constraint::Length(30),
          Constraint::Length(6)
        ]);
      f.render_widget(table, chunks[0]);


      let chart_block = Block::default()
            .title("Progress")
            .borders(Borders::ALL);
      f.render_widget(chart_block, chunks[1]);

      let details_block = Block::default()
          .title("Best layout")
          .borders(Borders::ALL);
      f.render_widget(details_block, main_chunks[1]);
    })?;

    match events::inst().next()? {
      Event::Result(outcome) => { // calculation result
        model.record(outcome);
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