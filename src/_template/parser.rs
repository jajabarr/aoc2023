use crate::_traits::parsing::{Parsing, _Parsing};

pub struct Parser {}

impl Parser {}

impl _Parsing<Parser> for Parser {
    fn read(&mut self, row: &usize, line: &str) {}
}

impl Parsing<Parser> for Parser {
    fn new(file_name: &str) -> Parser {
        let mut parser = Parser {};
        parser.parse(file_name);
        parser.solve();

        parser
    }

    fn solution_1(&self) {}

    fn solution_2(&self) {}

    fn solve(&self) {}
}
