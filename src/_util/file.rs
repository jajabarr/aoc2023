use std::fs::File;
use std::io::{self, BufRead};

pub fn read_lines(filename: &str, mut callback: impl FnMut(usize, String) -> ()) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    for (row, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => callback(row, line),
            _ => {}
        }
    }

    Ok(())
}
