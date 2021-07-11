use std::io;
use std::fs;

pub fn load(folder: String) -> Result<String, io::Error> {
  let pathname = format!("./sources/{}", folder);
  let filenames = fs::read_dir(pathname).unwrap();

  let mut contents = Vec::new();

  for path in filenames {
    let content = fs::read_to_string(path.unwrap().path().as_os_str())?;
    contents.push(content);
  }

  Ok(contents.join("\n\n"))
}

pub fn load_english_text() -> String {
  load(String::from("text")).unwrap()
}