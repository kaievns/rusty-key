use std::fs;

use once_cell::sync::Lazy;

use crate::source;
use crate::geometry::*;
use crate::preservative::*;
use crate::frequency::*;

pub const SAME_HAND_PENALTY: usize = 5;
pub const SAME_FINGER_PENALTY: usize = 100;
pub const BAD_STARTER_PENALTY: usize = 80;
pub const ROW_SKIP_PENALTY: usize = 50;
pub const ROW_JUMP_PENALTY: usize = 30;

pub const POPULATION_SIZE: usize = 50;
pub const MEMBERS_PER_MUTATION: usize = 10; // 4 batches, key symbol key symbol

pub static CONFIG: Lazy<Config> = Lazy::new(||{ Config::defaults() });

pub struct Config {
  pub geometry: Geometry,
  pub preserve: Preservative,
  pub data: String
}

impl Config {
  pub fn defaults() -> Config {
    let geometry = US_PC_KEYBOARD;
    let preserve = Preservative::from(load_preserve_template());
    let data = load_text();

    Config { geometry, preserve, data }
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