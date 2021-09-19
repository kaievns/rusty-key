use std::io;
use termion::raw::IntoRawMode;
use termion::event::Key;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::text::{Span};
use tui::style::{Color, Style};
use tui::symbols::{Marker};
use tui::widgets::{Block, Borders, Cell, Row, Table, Dataset, Chart, Axis, GraphType, Paragraph};
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
            .title(" Latest results ")
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


      let generation_number = match model.outcomes.last() {
        Some(outcome) => outcome.generation_number,
        None => 0
      };
      let chart_block = Block::default()
            .title(format!(" Progress (gen {}) ", generation_number))
            .borders(Borders::ALL);
      let top_scores = model.top_scores();
      let best_scores = model.best_scores();
      let winner_scores = model.winner_scores();
      let datasets = vec![
        Dataset::default()
          .name("Top scores")
          .marker(Marker::Braille)
          .graph_type(GraphType::Line)
          .style(Style::default().fg(Color::Cyan))
          .data(&top_scores),
        Dataset::default()
          .name("Best scores")
          .marker(Marker::Braille)
          .graph_type(GraphType::Line)
          .style(Style::default().fg(Color::Yellow))
          .data(&best_scores),
        Dataset::default()
          .name("Winner scores")
          .marker(Marker::Braille)
          .graph_type(GraphType::Line)
          .style(Style::default().fg(Color::Red))
          .data(&winner_scores)
      ];
      let highest_score = top_scores.last().unwrap_or(&(0.0, 10.0)).1;
      let chart = Chart::new(datasets)
          .block(chart_block)
          .y_axis(
            Axis::default()
              // .title("")
              .style(Style::default().fg(Color::Gray))
              .bounds([0.0, highest_score])
              .labels(vec![
                Span::raw("0"),
                Span::raw(format!("{}", (highest_score / 3.0) as usize)),
                Span::raw(format!("{}", (highest_score * 2.0 / 3.0) as usize)),
                Span::raw(format!("{}", highest_score as usize)),
            ])
          )
          .x_axis(
            Axis::default()
              // .title("")
              .style(Style::default().fg(Color::Gray))
              .bounds([0.0, top_scores.len() as f64])
          );

      f.render_widget(chart, chunks[1]);

      let details = match model.best_outcome() {
        None => String::from("No results yet"),
        Some(outcome) => format!(
          "\n{}\n\nSummary:\n  effort:      {:>6.2} ({:>2.0}%)\n  overheads:   {:>6.2} ({:>2.0}%)\n  awkwardness: {:>6.2} ({:>2.0}%)\n  rollingness: {:>6.2} ({:>2.0}%)\n  fitness:     {:>6.2} ({:>2.0}%)",
          outcome.best.template,
          outcome.best_summary.effort,
          100.0 * 9.0 / outcome.best_summary.effort,
          outcome.best_summary.overheads,
          100.0 * 7.0 / outcome.best_summary.overheads,
          outcome.best_summary.awkwardness,
          100.0 * 2.0 / outcome.best_summary.awkwardness,
          outcome.best_summary.rollingness,
          100.0 * outcome.best_summary.rollingness / 18.0,
          outcome.best_summary.fitness,
          100.0 * outcome.best_summary.fitness / 18.0
        )
      };

      let details_block = Block::default()
        .title(" Best layout ")
        .borders(Borders::ALL);
      let p = Paragraph::new(details)
        .block(details_block);
      f.render_widget(p, main_chunks[1]);
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