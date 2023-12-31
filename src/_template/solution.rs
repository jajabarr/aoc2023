use super::parser::Parser;
use super::{INPUT_FILE, SAMPLE_FILE};
use crate::_util::file::read_lines;

pub fn solve() -> () {
    let mut parser = Parser::new(INPUT_FILE);

    println!("Solution 1: {}", parser.solution_1());
    println!("Solution 2: {}", parser.solution_2());
}
