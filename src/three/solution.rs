use super::parser::Parser;
use super::{INPUT_FILE, SAMPLE_FILE};
use crate::_util::file::read_lines;

pub fn solve() -> () {
    let mut parser = Parser::new();

    if let Err(_) = read_lines(INPUT_FILE, |row, line| {
        parser.read(row, &line);
    }) {
        println!("Error reading file");
    }

    parser.find_machine_parts();
    let mut sum = 0;
    for part in parser.machine_parts.iter() {
        sum += part.value;
    }

    println!("solution: {}", sum);
}
