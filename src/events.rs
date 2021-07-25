use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;

use termion::event::Key;
use termion::input::TermRead;

use once_cell::sync::OnceCell;

use crate::generation::Outcome;

pub fn inst() -> &'static Events {
  static EVENTS: OnceCell<Events> = OnceCell::new();
  EVENTS.get_or_init(|| { Events::new() })
}

pub enum Event {
  Result(Outcome),
  Input(Key),
  Tick,
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
  tx: Mutex<mpsc::Sender<Event>>,
  rx: Mutex<mpsc::Receiver<Event>>,
  input_handle: thread::JoinHandle<()>,
  tick_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
  pub tick_rate: Duration,
}

impl Default for Config {
  fn default() -> Config {
    Config {
      tick_rate: Duration::from_millis(250),
    }
  }
}

impl Events {
  pub fn new() -> Events {
    Events::with_config(Config::default())
  }

  pub fn with_config(config: Config) -> Events {
    let (tx, rx) = mpsc::channel();
    let input_handle = {
      let tx = tx.clone();
      thread::spawn(move || {
        let stdin = io::stdin();
        for evt in stdin.keys() {
          if let Ok(key) = evt {
            if let Err(err) = tx.send(Event::Input(key)) {
              eprintln!("{}", err);
              return;
            }
          }
        }
      })
    };
    let tick_handle = {
      let tx = tx.clone();
      thread::spawn(move || loop {
        if let Err(err) = tx.send(Event::Tick) {
          eprintln!("{}", err);
          break;
        }
        thread::sleep(config.tick_rate);
      })
    };
    Events {
      rx: Mutex::new(rx),
      tx: Mutex::new(tx),
      input_handle,
      tick_handle,
    }
  }

  pub fn send_result(&self, outcome: &Outcome) {
    let tx = self.tx.lock().unwrap();
    tx.send(Event::Result((*outcome).clone()));
  }

  pub fn next(&self) -> Result<Event, mpsc::RecvError> {
    let rx = self.rx.lock().unwrap();
    rx.recv()
  }
}