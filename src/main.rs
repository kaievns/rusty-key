mod source;
mod keyboard;


fn main() -> Result<(), std::io::Error> {
  let data = source::load(String::from("text"))?;
  println!("Loaded text: {:}", data.len());

  println!("QUERTY: {:?}", keyboard::Keyboard::querty());

  Ok(())
}
