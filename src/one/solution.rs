use super::parser::Parser;
use crate::_util::file::read_lines;

fn find_line_result(line: &String) -> i32 {
    let mut parser = Parser::new();
    let mut numbers = String::new();
    for item in line.chars() {
        if let Some(result) = parser.check(&item) {
            if numbers.len() >= 2 {
                numbers.pop();
                numbers.push(result);
                continue;
            }
            numbers.push(result);
        }
    }

    if numbers.len() == 2 {
        return numbers.parse().unwrap();
    }

    if let Some(first) = numbers.chars().next() {
        numbers.push(first);
    }

    numbers.parse().unwrap()
}

pub fn solve() -> () {
    let mut total = 0;

    if let Err(_) = read_lines("src/one/input.txt", |_, line| {
        total += find_line_result(&line);
    }) {
        println!("Error reading file");
    }

    println!("Total: {}", total);
}
