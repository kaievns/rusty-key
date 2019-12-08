mod source;


fn main() -> std::io::Result<()> {
  let data = source::load(String::from("text"))?;

  println!("{:}", data);

  Ok(())
}
