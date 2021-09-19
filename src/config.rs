use std::fs;

use toml;
use serde::Deserialize;
use once_cell::sync::Lazy;

use crate::source;
use crate::geometry::*;
use crate::preservative::*;

pub const SAME_HAND_PENALTY: usize = 5;
pub const SAME_FINGER_PENALTY: usize = 100;
pub const BAD_STARTER_PENALTY: usize = 80;
pub const ROW_SKIP_PENALTY: usize = 50;
pub const ROW_JUMP_PENALTY: usize = 30;

pub static CONFIG: Lazy<Config> = Lazy::new(||{ Config::defaults() });

pub struct Config<'a> {
  pub geometry: &'a Geometry,
  pub preserve: Preservative,
  pub data: String,
  pub progress_window_size: usize,
  pub population: PopulationConfig,
  pub selection: SelectionConfig,
  pub weights: WeightsConfig
}

#[derive(Deserialize,Debug)]
pub struct ExternalConfig {
  pub geometry: String,
  pub progress_window_size: usize,
  pub population: PopulationConfig,
  pub selection: SelectionConfig,
  pub weights: WeightsConfig
}

#[derive(Deserialize,Debug)]
pub struct WeightsConfig {
  pub effort: usize,
  pub overheads: usize,
  pub awkwardness: usize,
  pub rollingness: usize,
  pub fitness: usize
}

#[derive(Deserialize,Debug)]
pub struct SelectionConfig {
  pub rank_space_cut_off: usize,
  pub diversity_bias: f64
}

#[derive(Deserialize,Debug)]
pub struct PopulationConfig {
  pub size: usize,
  pub steps: usize,
  pub elites: usize,
  pub symbols: bool
}

impl Config<'_> {
  pub fn defaults() -> Config<'static> {
    let config = load_external_config();
    let geometry = if config.geometry == String::from("ORTHO") { &*FULL_ORTHO } else { &*US_PC_KEYBOARD };
    let preserve = Preservative::from(load_preserve_template());
    let data = load_text();

    Config { 
      geometry, 
      preserve, 
      data,
      progress_window_size: config.progress_window_size,
      population: config.population,
      selection: config.selection,
      weights: config.weights
    }
  }
}

fn load_external_config() -> ExternalConfig {
  if cfg!(test) { default_config() }
  else {
    let data = fs::read_to_string("./config.toml").unwrap_or(String::from(""));
    toml::from_str(&data).unwrap_or(default_config())
  }
}

fn default_config() -> ExternalConfig {
  ExternalConfig {
    geometry: "US-PC".to_string(),
    progress_window_size: 200,
    population: PopulationConfig {
      size: 30,
      steps: 3,
      elites: 30,
      symbols: true
    },
    selection: SelectionConfig {
      rank_space_cut_off: 50,
      diversity_bias: 1.0
    },
    weights: WeightsConfig {
      effort: 1,
      overheads: 1,
      awkwardness: 1,
      rollingness: 1,
      fitness: 1
    }
  }
}

fn load_preserve_template() -> String {
  if cfg!(test) { String::from("") }
  else {
    fs::read_to_string("./preserve.txt").unwrap_or(String::from(""))
  }
}

fn load_text() -> String {
  if cfg!(test) { lorem_ipsum() }
  else { source::load_english_text() }
}

fn lorem_ipsum() -> String {
  "
  Lorem Ipsum è un testo segnaposto utilizzato nel settore della tipografia e della stampa. 
  Lorem Ipsum è considerato il testo segnaposto standard sin dal sedicesimo secolo, 
  quando un anonimo tipografo prese una cassetta di caratteri e li assemblò per preparare 
  un testo campione. È sopravvissuto non solo a più di cinque secoli, ma anche al passaggio 
  alla videoimpaginazione, pervenendoci sostanzialmente inalterato. Fu reso popolare, negli 
  anni ’60, con la diffusione dei fogli di caratteri trasferibili “Letraset”, che contenevano 
  passaggi del Lorem Ipsum, e più recentemente da software di impaginazione come Aldus PageMaker, 
  che includeva versioni del Lorem Ipsum.
  ".to_string()
}