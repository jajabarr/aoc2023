use super::parser::Parser;
use crate::_util::file::read_lines;

pub fn solve() -> () {
    let parser = Parser::new();
    let mut solution: i32 = 0;
    let mut solution_2: i32 = 0;

    if let Err(_) = read_lines("src/two/input.txt", |_, line| {
        let game = parser.from(&line);

        solution_2 += game.red * game.green * game.blue;

        if game.red > 12 || game.green > 13 || game.blue > 14 {
            println!("\t> {}", game);
            return;
        }

        println!("{}", game);
        solution += game.id;
    }) {
        println!("Error reading file");
    }

    println!("solutions: {}, {}", solution, solution_2);
}
