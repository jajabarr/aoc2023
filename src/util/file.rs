use std::fs::File;
use std::io::{self, BufRead};

pub fn read_lines(filename: &str, mut callback: impl FnMut(String) -> ()) -> io::Result<()> {
  let file = File::open(filename)?;
  let reader = io::BufReader::new(file);

  for line in reader.lines() {
      match line { 
        Ok(line) => callback(line),
        _ => {},
      }
  }

  Ok(())
}
