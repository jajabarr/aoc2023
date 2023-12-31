use crate::_util::file::read_lines;

pub trait _Parsing<T> {
    fn parse(&mut self, file_name: &str) {
        if let Err(_) = read_lines(file_name, |row, line| {
            self.read(&row, &line);
        }) {
            println!("Error reading file");
        }
    }

    fn read(&mut self, row: &usize, line: &str);
}

pub trait Parsing<T> {
    fn new(file_name: &str) -> T;

    fn solution_1(&self) {}

    fn solution_2(&self) {}

    fn solve(&self);
}
