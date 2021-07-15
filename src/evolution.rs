/**
 * This module represents the evolution model
 */
use std::cell::RefCell;
use std::sync::{Arc,Mutex};

use crate::layout::*;
use crate::generation::*;

type Generations = Vec<Generation>;
type Winners = Vec<(Layout, usize)>;

pub struct Evolution {
  in_progress: Arc<Mutex<RefCell<bool>>>,
  current_generation: Arc<Mutex<RefCell<Generation>>>,
  past_generations: Arc<Mutex<Generations>>
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

  fn current_layout(&self) -> Layout {
    let guard = self.past_generations.lock().unwrap();
    let generation = &(*guard)[0];
    generation.population.members[0].clone()
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

    println!("tick! {:?}", current_generation.number);

    let past_gens = &mut *past.lock().unwrap();
    past_gens.push(current_generation);
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::time;

  // #[test]
  // fn test_new() {
  //   let evolution = Evolution::new();
  //   let current_generation = evolution.current_generation.borrow();

  //   assert_eq!(evolution.in_progress, RefCell::new(false));
  //   assert_eq!(current_generation.population.members[0].template, QWERTY);
  // }

  #[test]
  fn test_start() {
    println!("creating");
    let evolution = Evolution::new();

    println!("starting");
    evolution.start();

    // assert_eq!(evolution.in_progress, RefCell::new(true));
    println!("sleeping");
    std::thread::sleep(time::Duration::from_millis(1000));
    println!("current {:?}", evolution.current_layout());
  
    println!("stopping");
    evolution.stop();

    std::thread::sleep(time::Duration::from_millis(500));

    // handle.join();

    println!("asserting");
    // assert_eq!(evolution.in_progress, RefCell::new(false));
    // assert_eq!(evolution.inner.lock().unwrap().past_generations.len(), 1234);
    assert!(false);
  }

}