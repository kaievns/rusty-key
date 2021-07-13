/**
 * Okay, so this one is a bit weird one. Basically what we're doing here is this
 *
 * 1. Turning all values into coefficients against the max value in a bunch
 * 2. Turning the coefficients into a set of distances from the max possible value (1, 1, 1)
 * 3. Reordering all values based on those distances (shortest distances on top)
 * 4. Use degrading probability to pick the next fittest score
 */
use rand::Rng;
use core::cmp::Ordering::Less;

#[derive(PartialEq,Debug)]
pub struct Score {
  pub performance: f64,
  pub deviation: f64,
  pub fitness: f64
}

pub type Scores = Vec<Score>;
type RankSpace = Vec<(usize, f64)>;

const RANK_SPACE_CUT_OFF: usize = 50; // %

pub struct Selection {
  pub scores: Scores
}

impl Selection {
  pub fn select_the_fittest(self: &Self) -> &Score {
    let rank_space = self.create_rank_space();
    let (index, _) = self.select_from_rank_space(&rank_space);
    &self.scores[index]
  }

  fn select_from_rank_space(self: &Self, list: &RankSpace) -> (usize, f64) {
    let mut rng = rand::thread_rng();
    let lucky = rng.gen_range(0..100) < RANK_SPACE_CUT_OFF;

    if lucky || list.len() == 1 {
      *list.first().unwrap()
    } else {
      self.select_from_rank_space(&(&list[1..]).to_vec())
    }
  }

  fn create_rank_space(self: &Self) -> RankSpace {
    let mut rank_space: RankSpace = self.calculate_ranks()
      .iter().enumerate().map(|(i, r)| (i, *r)).collect();

    rank_space.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Less));
    rank_space
  }

  fn calculate_ranks(self: &Self) -> Vec<f64> {
    self.renormalise().iter().map(|score| {
      // recalculating from the top right corner
      let x = (1.0 - score.performance).powf(2.0);
      let y = (1.0 - score.deviation).powf(2.0);
      let z = (1.0 - score.fitness).powf(2.0);
      
      (x + y + z).sqrt() // distance
    })
    .collect()
  }

  fn renormalise(self: &Self) -> Scores {
    let max_performance = self.scores.iter().map(|s| s.performance).fold(0./0., f64::max);
    let max_deviation = self.scores.iter().map(|s| s.deviation).fold(0./0., f64::max);
    let max_fitness = self.scores.iter().map(|s| s.fitness).fold(0./0., f64::max);

    self.scores.iter().map(|s| Score {
      performance: s.performance / max_performance,
      deviation: s.deviation / max_deviation,
      fitness: s.fitness / max_fitness
    })
    .collect()
  }
}

#[cfg(test)]
mod test {
  use super::*;

  fn get_scores() -> Scores {
    vec![
      Score { performance: 1.1, deviation: 0.3, fitness: 0.2 },
      Score { performance: 1.2, deviation: 0.5, fitness: 0.4 },
      Score { performance: 1.3, deviation: 0.8, fitness: 0.5 },
      Score { performance: 1.5, deviation: 1.1, fitness: 0.7 }
    ]
  }

  #[test]
  fn test_selection_of_the_fittest() {
    let selection = Selection { scores: get_scores() };
    let mut the_fittest_was_selected = 0;

    for _ in 0..1000 {
      let selected = selection.select_the_fittest();
      let the_fittest = selection.scores[3]
      println!("{:?}", selected);
      if *selected == the_fittest {
        the_fittest_was_selected += 1;
      }
    }

    // the last (fittest) was returned half of the times
    assert!((450..550).contains(&the_fittest_was_selected));
  }

  #[test]
  fn test_rank_space_selection() {
    let selection = Selection { scores: get_scores() };
    let rank_space = selection.create_rank_space();

    let first = rank_space[0];
    let second = rank_space[1];
    let third = rank_space[2];
    let fourth = rank_space[3];

    let mut first_selected = 0;
    let mut second_selected = 0;
    let mut third_selected = 0;
    let mut fourth_selected = 0;

    for _ in 0..1000 {
      let selected = selection.select_from_rank_space(&rank_space);
      if selected == first { first_selected += 1; }
      else if selected == second { second_selected += 1; }
      else if selected == third { third_selected += 1; }
      else if selected == fourth { fourth_selected += 1; }
      else { panic!("Unexpected item returned"); }
    }

    println!("{:?} {:?} {:?} {:?}", first_selected, second_selected, third_selected, fourth_selected);

    assert!((450..550).contains(&first_selected));
    assert!((200..300).contains(&second_selected));
    assert!((50..150).contains(&third_selected));
    assert!((50..150).contains(&fourth_selected));
  }

  #[test]
  fn test_creation_of_rank_space() {
    let sel = Selection { scores: get_scores() };

    assert_eq!(sel.create_rank_space(), vec![
      (3, 0.0), 
      (2, 0.4168819930487025), 
      (1, 0.7219377608525404), 
      (0, 1.0536796536796535)
    ])
  }

  #[test]
  fn test_ratings_calculation() {
    let sel = Selection { scores: get_scores() };

    assert_eq!(sel.calculate_ranks(), vec![
      1.0536796536796535, 
      0.7219377608525404, 
      0.4168819930487025, 
      0.0
    ])
  }
  
  #[test]
  fn test_renormalise() {
    let sel = Selection { scores: get_scores() };

    assert_eq!(sel.renormalise(), vec![
      Score { performance: 0.7333333333333334, deviation: 0.2727272727272727, fitness: 0.28571428571428575 }, 
      Score { performance: 0.7999999999999999, deviation: 0.45454545454545453, fitness: 0.5714285714285715 }, 
      Score { performance: 0.8666666666666667, deviation: 0.7272727272727273, fitness: 0.7142857142857143 }, 
      Score { performance: 1.0, deviation: 1.0, fitness: 1.0 }
    ])
  }
}