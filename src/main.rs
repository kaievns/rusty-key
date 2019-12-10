mod source;
mod ui;


fn main() -> Result<(), std::io::Error> {
  let data = source::load(String::from("text"))?;
  println!("Loaded text: {:}", data.len());

  ui::render()?;

  Ok(())
}
