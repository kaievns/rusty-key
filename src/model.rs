/* this is the view-model for the ui */

use crate::config::CONFIG;
use crate::summary::Summary;
use crate::generation::Outcome;

pub struct ViewModel {
  pub outcomes: Vec<Outcome>
}

fn to_score(summary: &Summary) -> usize {
  (summary.score() * 1000.0) as usize
}

impl ViewModel {
  pub fn new() -> ViewModel {
    ViewModel { outcomes: vec![] }
  }

  pub fn record(&mut self, outcome: Outcome) {
    self.outcomes.push(outcome);
  }
  
  pub fn top_list(&self) -> Vec<Vec<String>> {
    self.sorted_outcomes().iter().enumerate().map(|(i, outcome)|
      vec![
        format!("{}", i+1), 
        outcome.best.long_name(), 
        format!("{}", to_score(&outcome.best_summary))
      ]
    ).collect()
  }

  pub fn best_outcome(&self) -> Option<Outcome> {
    match self.sorted_outcomes().get(0) {
      Some(outcome) => Some(outcome.clone()),
      None => None
    }
  }

  // best scores of all time
  pub fn top_scores(&self) -> Vec<(f64, f64)> {
    let mut list = self.sorted_outcomes().clone();
    list.reverse(); // historical order
    list.iter().rev().take(CONFIG.progress_window_size).rev()
    .enumerate().map(|(i, outcome)|
      (i as f64, to_score(&outcome.best_summary) as f64)
    ).collect()
  }

  // best scores as they come
  pub fn best_scores(&self) -> Vec<(f64, f64)> {
    self.outcomes.iter().rev().take(CONFIG.progress_window_size).rev()
    .enumerate().map(|(i, outcome)|
      (i as f64, to_score(&outcome.best_summary) as f64)
    ).collect()
  }

  pub fn winner_scores(&self) -> Vec<(f64, f64)> {
    self.outcomes.iter().rev().take(CONFIG.progress_window_size).rev()
    .enumerate().map(|(i, outcome)|
      (i as f64, to_score(&outcome.winner_summary) as f64)
    ).collect()
  }

  fn sorted_outcomes(&self) -> Vec<Outcome> {
    let mut list = self.outcomes.clone();
    list.sort_by_key(|o| to_score(&o.best_summary));
    list.reverse();
    list
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::layout::*;

  #[test]
  fn test_instance() {
    let model = ViewModel::new();
    assert_eq!(model.outcomes, vec![]);
  }

  #[test]
  fn test_push() {
    let mut model = ViewModel::new();
    let outcome = Outcome {
      best: COLEMAK.clone(),
      best_summary: Summary {
        effort: 1.234,
        overheads: 1.234,
        awkwardness: 1.234,
        rollingness: 1.234,
        fitness: 1.234
      },
      winner: WORKMAN.clone(),
      winner_summary: Summary {
        effort: 1.234,
        overheads: 1.234,
        awkwardness: 1.234,
        rollingness: 1.234,
        fitness: 1.234
      }
    };
    model.record(outcome);

    assert_eq!(model.outcomes.len(), 1);
  }

  #[test]
  fn top_list() {
    let mut model = ViewModel::new();

    assert_eq!(model.top_list().len(), 0);

    model.record(Outcome {
      best: COLEMAK.clone(),
      best_summary: Summary {
        effort: 1.234,
        overheads: 1.234,
        awkwardness: 1.234,
        rollingness: 1.234,
        fitness: 1.234
      },
      winner: WORKMAN.clone(),
      winner_summary: Summary {
        effort: 1.234,
        overheads: 1.234,
        awkwardness: 1.234,
        rollingness: 1.234,
        fitness: 1.234
      }
    });

    assert_eq!(model.top_list(), vec![
      vec!["1".to_string(), "QWFPGJLUY;".to_string(), "20629".to_string()]
    ]);

    model.record(Outcome {
      best: WORKMAN.clone(),
      best_summary: Summary {
        effort: 0.234,
        overheads: 0.234,
        awkwardness: 0.234,
        rollingness: 2.234,
        fitness: 2.234
      },
      winner: WORKMAN.clone(),
      winner_summary: Summary {
        effort: 1.234,
        overheads: 1.234,
        awkwardness: 1.234,
        rollingness: 1.234,
        fitness: 1.234
      }
    });

    assert_eq!(model.top_list(), vec![
      vec!["1".to_string(), "QDRWBJFUP;".to_string(), "107507".to_string()], 
      vec!["2".to_string(), "QWFPGJLUY;".to_string(), "20629".to_string()]
    ]);
  }

  #[test]
  fn best_outcome() {
    let mut model = ViewModel::new();

    assert_eq!(model.best_outcome(), None);

    model.record(Outcome {
      best: COLEMAK.clone(),
      best_summary: Summary {
        effort: 1.234,
        overheads: 1.234,
        awkwardness: 1.234,
        rollingness: 1.234,
        fitness: 1.234
      },
      winner: WORKMAN.clone(),
      winner_summary: Summary {
        effort: 1.234,
        overheads: 1.234,
        awkwardness: 1.234,
        rollingness: 1.234,
        fitness: 1.234
      }
    });

    assert_eq!(model.best_outcome(), Some(model.outcomes[0].clone()));

    model.record(Outcome {
      best: WORKMAN.clone(),
      best_summary: Summary {
        effort: 0.234,
        overheads: 0.234,
        awkwardness: 0.234,
        rollingness: 2.234,
        fitness: 2.234
      },
      winner: WORKMAN.clone(),
      winner_summary: Summary {
        effort: 1.234,
        overheads: 1.234,
        awkwardness: 1.234,
        rollingness: 1.234,
        fitness: 1.234
      }
    });

    assert_eq!(model.best_outcome(), Some(model.outcomes[1].clone()));
  }

  #[test]
  fn top_scores() {
    let mut model = ViewModel::new();

    assert_eq!(model.top_scores().len(), 0);

    model.record(Outcome {
      best: COLEMAK.clone(),
      best_summary: Summary {
        effort: 1.234,
        overheads: 1.234,
        awkwardness: 1.234,
        rollingness: 1.234,
        fitness: 1.234
      },
      winner: WORKMAN.clone(),
      winner_summary: Summary {
        effort: 1.234,
        overheads: 1.234,
        awkwardness: 1.234,
        rollingness: 1.234,
        fitness: 1.234
      }
    });

    assert_eq!(model.top_scores(), vec![(0.0, 20629.0)]);

    model.record(Outcome {
      best: WORKMAN.clone(),
      best_summary: Summary {
        effort: 0.234,
        overheads: 0.234,
        awkwardness: 0.234,
        rollingness: 2.234,
        fitness: 2.234
      },
      winner: WORKMAN.clone(),
      winner_summary: Summary {
        effort: 1.234,
        overheads: 1.234,
        awkwardness: 1.234,
        rollingness: 1.234,
        fitness: 1.234
      }
    });

    assert_eq!(model.top_scores(), vec![
      (0.0, 20629.0), (1.0, 107507.0)
    ]);
  }

  #[test]
  fn best_scores() {
    let mut model = ViewModel::new();

    assert_eq!(model.best_scores().len(), 0);

    model.record(Outcome {
      best: COLEMAK.clone(),
      best_summary: Summary {
        effort: 1.234,
        overheads: 1.234,
        awkwardness: 1.234,
        rollingness: 1.234,
        fitness: 1.234
      },
      winner: WORKMAN.clone(),
      winner_summary: Summary {
        effort: 1.234,
        overheads: 1.234,
        awkwardness: 1.234,
        rollingness: 1.234,
        fitness: 1.234
      }
    });

    assert_eq!(model.best_scores(), vec![(0.0, 20629.0)]);

    model.record(Outcome {
      best: WORKMAN.clone(),
      best_summary: Summary {
        effort: 2.234,
        overheads: 2.234,
        awkwardness: 2.234,
        rollingness: 1.234,
        fitness: 1.234
      },
      winner: WORKMAN.clone(),
      winner_summary: Summary {
        effort: 1.234,
        overheads: 1.234,
        awkwardness: 1.234,
        rollingness: 1.234,
        fitness: 1.234
      }
    });

    assert_eq!(model.best_scores(), vec![
      (0.0, 20629.0), (1.0, 11560.0)
    ]);
  }

  #[test]
  fn winner_scores() {
    let mut model = ViewModel::new();

    assert_eq!(model.winner_scores().len(), 0);

    model.record(Outcome {
      best: COLEMAK.clone(),
      best_summary: Summary {
        effort: 1.234,
        overheads: 1.234,
        awkwardness: 1.234,
        rollingness: 1.234,
        fitness: 1.234
      },
      winner: WORKMAN.clone(),
      winner_summary: Summary {
        effort: 2.34,
        overheads: 1.234,
        awkwardness: 1.234,
        rollingness: 1.234,
        fitness: 1.234
      }
    });

    assert_eq!(model.winner_scores(), vec![(0.0, 16799.0)]);

    model.record(Outcome {
      best: WORKMAN.clone(),
      best_summary: Summary {
        effort: 0.234,
        overheads: 0.234,
        awkwardness: 0.234,
        rollingness: 2.234,
        fitness: 2.234
      },
      winner: WORKMAN.clone(),
      winner_summary: Summary {
        effort: 2.34,
        overheads: 2.34,
        awkwardness: 1.234,
        rollingness: 1.234,
        fitness: 1.234
      }
    });

    assert_eq!(model.winner_scores(), vec![
      (0.0, 16799.0), (1.0, 12969.0)
    ]);
  }
}