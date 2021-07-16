/**
 * This module represents the evolution model
 */
use std::cell::RefCell;
use std::sync::{Arc,Mutex};

use crate::generation::*;

type Generations = Vec<Generation>;

pub struct Evolution {
  in_progress: Arc<Mutex<RefCell<bool>>>,
  current_generation: Arc<Mutex<RefCell<Generation>>>,
  pub past_generations: Arc<Mutex<Generations>>
}

impl Evolution {
  pub fn new() -> Evolution {
    let in_progress = Arc::new(Mutex::new(RefCell::new(false)));
    let current_generation = Arc::new(Mutex::new(RefCell::new(Generation::zero())));
    let past_generations = Arc::new(Mutex::new(Generations::new()));

    Evolution { in_progress, current_generation, past_generations }
  }

  pub fn start(&self) {
    let flag = &mut *self.in_progress.lock().unwrap();
    flag.replace(true);
  
    Evolution::start_thread(
      self.in_progress.clone(),
      self.current_generation.clone(),
      self.past_generations.clone()
    );
  }

  pub fn stop(&self) {
    let flag = &mut *self.in_progress.lock().unwrap();
    flag.replace(false);
  }

  fn start_thread(
    flag: Arc<Mutex<RefCell<bool>>>,
    current: Arc<Mutex<RefCell<Generation>>>,
    past: Arc<Mutex<Generations>>
  ) {
    std::thread::spawn(move || {
      loop {
        if Evolution::fetch_status(&flag) {            
          let next_generation = Evolution::get_next(&current);
          Evolution::swap(&current, &past, next_generation);
        } else {
          break;
        }
      }
    });
  }

  fn fetch_status(mutex: &Arc<Mutex<RefCell<bool>>>) -> bool {
    let guard = &*mutex.lock().unwrap();
    let val = guard.borrow();
    *val
  }

  fn get_next(current: &Arc<Mutex<RefCell<Generation>>>) -> Generation {
    let guard = &mut *current.lock().unwrap();
    let next_gen = guard.borrow().next();
    next_gen
  }

  fn swap(
    current: &Arc<Mutex<RefCell<Generation>>>,
    past: &Arc<Mutex<Generations>>,
    next_generation: Generation
  ) {
    let guard = &mut *current.lock().unwrap();
    let current_generation = guard.replace(next_generation);

    let past_gens = &mut *past.lock().unwrap();
    past_gens.push(current_generation);
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::time;
  use crate::layout::*;

  #[test]
  fn test_new() {
    let evolution = Evolution::new();
    assert_eq!(Evolution::fetch_status(&evolution.in_progress), false);

    let guard = &*evolution.current_generation.lock().unwrap();
    let current_generation = guard.borrow();

    assert_eq!(current_generation.population.members[0].template, QWERTY);
  }

  #[test]
  fn test_start() {
    let evolution = Evolution::new();
    assert_eq!(Evolution::fetch_status(&evolution.in_progress), false);

    evolution.start();
    assert_eq!(Evolution::fetch_status(&evolution.in_progress), true);

    std::thread::sleep(time::Duration::from_millis(200));
    evolution.stop();
    assert_eq!(Evolution::fetch_status(&evolution.in_progress), false);
  }

}